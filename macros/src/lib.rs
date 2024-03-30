use proc_macro::TokenStream;
use syn::{ fold::Fold, Expr, ItemFn, Signature, ReturnType };
use syn::{ fold, parse_macro_input, parse_quote };
use quote::quote;


//---------------------------------------------------------
//  Macros
//---------------------------------------------------------


#[proc_macro_attribute]
pub fn generator(attr: TokenStream, tokens: TokenStream) -> TokenStream {
	let function = parse_macro_input!(tokens as ItemFn);
	let boxed = attr.to_string().eq("boxed");

	// Get Return Type
	let ReturnType::Type(arrow, return_type) = &function.sig.output else {
		panic!("Generator functions must have an explicit return type");
	};

	// Expand Return Type
	let output = parse_quote!{ impl Iterator<Item = #return_type> };
	let output = ReturnType::Type(*arrow, Box::new(output));
	let sig    = Signature { output, ..function.sig };

	// Expand Function Body
	let block = Transforms.fold_block(*function.block);
	let block = match boxed {
		true  => parse_quote!{{ generators::Generator(Box::new(move || #block)) }},
		false => parse_quote!{{ generators::Generator(move || #block) }}
	};

	// Return New Function
	let expanded = ItemFn { sig, block, ..function };
	TokenStream::from(quote!(#expanded))
}


#[proc_macro]
pub fn yield_try(tokens: TokenStream) -> TokenStream {
	let expr = parse_macro_input!(tokens as Expr);

	// Expand Into Try
	let expanded: syn::Block = parse_quote! {{
		use core::ops::{ Try, FromResidual, ControlFlow::* };
		match Try::branch(#expr) {
			Continue(value) => value,
			Break(residual) => return yield <_ as FromResidual>::from_residual(residual)
		}
	}};

	TokenStream::from(quote!(#expanded))
}


#[proc_macro]
pub fn yield_from(tokens: TokenStream) -> TokenStream {
	let expr = parse_macro_input!(tokens as Expr);

	// Expand Into Loop
	let expanded: syn::ExprForLoop = match expr {

		// Try Expressions
		Expr::Try(syn::ExprTry { expr, .. }) => {
			parse_quote! { for x in #expr { yield generators::yield_try!(x) }}
		}

		// Other Expression
		expr => {
			parse_quote! { for x in #expr { yield x }}
		}
	};

	TokenStream::from(quote!(#expanded))
}


//---------------------------------------------------------
//  Recursive Transforms
//---------------------------------------------------------


struct Transforms;
impl Fold for Transforms {

	// Skip Folding Inner Functions
	fn fold_item_fn(&mut self, function: ItemFn) -> ItemFn {
		function
	}

	// Fold Expressions
	fn fold_expr(&mut self, expr: Expr) -> Expr { match expr {

		// Return Expressions
		Expr::Return(syn::ExprReturn { expr: Some(expr), .. }) => {
			let expr = self.fold_expr(*expr);
			parse_quote! { return yield #expr }
		}

		// Try Expressions
		Expr::Try(syn::ExprTry { expr, .. }) => {
			let expr = self.fold_expr(*expr);
			parse_quote! { generators::yield_try!(#expr) }
		}

		// Other Expressions
		expr => fold::fold_expr(self, expr)
	}}

}