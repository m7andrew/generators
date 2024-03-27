use proc_macro::TokenStream;
use syn::{ fold::Fold, Expr, ItemFn, Signature, ReturnType };
use syn::{ fold, parse_macro_input, parse_quote };
use quote::quote;

//---------------------------------------------------------
//  Macros
//---------------------------------------------------------


#[proc_macro_attribute]
pub fn generator(attr: TokenStream, tokens: TokenStream) -> TokenStream {

	// Parse Input
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
		true  => parse_quote!{{ Generator(Box::new(move || #block)) }},
		false => parse_quote!{{ Generator(move || #block) }}
	};

	// Return New Function
	let generator = ItemFn { sig, block, ..function };
	TokenStream::from(quote!(#generator))
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
			parse_quote! { match #expr {
				Ok(value)  => value,
				Err(error) => return yield Err(error)
			}}
		}

		// Other Expressions
		expression => fold::fold_expr(self, expression)
	}}

}