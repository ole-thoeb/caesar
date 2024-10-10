use crate::ast::{DeclKind, DeclRef, DomainDecl, DomainSpec, FuncDecl, Ident, Param, Span, Spanned, Symbol, TyKind, VarDecl, VarKind};
use crate::tyctx::TyCtx;
use std::cell::RefCell;
use std::ops::Deref;

/// Initializes Fuel domain for this context
pub fn init_limited(tcx: &mut TyCtx) {
    let fuel_name = fuel_ident();
    let z_name = Ident::with_dummy_span(Symbol::intern("Z"));
    let s_name = Ident::with_dummy_span(Symbol::intern("S"));

    let mut fuel_domain = DeclRef::new(DomainDecl {
        name: fuel_name,
        span: Span::dummy_span(),
        body: vec![],
    });
    let fuel_domain_ty = TyKind::Domain(fuel_domain.clone());

    let z_function = DeclRef::new(FuncDecl {
        name: z_name,
        span: Span::dummy_span(),
        inputs: Spanned::with_dummy_span(vec![]),
        output: fuel_domain_ty.clone(),
        body: RefCell::new(None),
    });
    let s_function = DeclRef::new(FuncDecl {
        name: s_name,
        span: Span::dummy_span(),
        inputs: Spanned::with_dummy_span(vec![Param {
            name: Ident::with_dummy_span(Symbol::intern("f")),
            span: Span::dummy_span(),
            literal_only: false,
            ty: Box::new(fuel_domain_ty.clone()),
        }]),
        output: fuel_domain_ty.clone(),
        body: RefCell::new(None),
    });

    fuel_domain.borrow_mut().body.push(DomainSpec::Function(z_function.clone()));
    fuel_domain.borrow_mut().body.push(DomainSpec::Function(s_function.clone()));

    tcx.declare(DeclKind::DomainDecl(fuel_domain));
    tcx.declare(DeclKind::FuncDecl(z_function));
    tcx.declare(DeclKind::FuncDecl(s_function));
    tcx.declare(DeclKind::VarDecl(DeclRef::new(VarDecl {
        name: Ident::with_dummy_span(Symbol::intern("f")),
        ty: fuel_domain_ty.clone(),
        kind: VarKind::Input,
        span: Span::dummy_span(),
        init: None,
        created_from: None,
    })));
    tcx.declare(DeclKind::VarDecl(DeclRef::new(VarDecl {
        name: Ident::with_dummy_span(Symbol::intern("fuel")),
        ty: fuel_domain_ty.clone(),
        kind: VarKind::Quant,
        span: Span::dummy_span(),
        init: None,
        created_from: None,
    })));

    tcx.add_global(fuel_name);
    tcx.add_global(z_name);
    tcx.add_global(s_name);
}

pub fn fuel_ident() -> Ident {
    Ident::with_dummy_span(Symbol::intern("Fuel"))
}

pub fn get_fuel_domain(tcx: &TyCtx) -> DeclRef<DomainDecl> {
    let decl = tcx.get(fuel_ident()).expect("Fuel domain to be initialized");
    match decl.deref() {
        DeclKind::DomainDecl(domain_decl) => domain_decl.clone(),
        _ => panic!("expected domain declaration")
    }
}