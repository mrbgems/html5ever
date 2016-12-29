#[macro_use] extern crate mrusty;
extern crate tendril;
extern crate html5ever;
#[macro_use] extern crate html5ever_atoms;
extern crate selectors;

use html5ever::ParseOpts;
use html5ever::{parse_fragment, parse_document, serialize};
use html5ever::rcdom::{Handle, NodeEnum, RcDom, ElementEnum};

use mrusty::{MrubyFile, MrubyImpl, MrValue, Value};

use tendril::TendrilSink;


struct RcDomWrapper {
  value: Handle,
}

fn check_parse_error(mrb: mrusty::MrubyType, str: &str, dom: RcDom) -> Value {
  if !dom.errors.is_empty() {
    println!("html5ever parse error (`{}`):\n  {}", str, dom.errors.join("\n  "));
    mrb.nil()
  } else {
    mrb.obj(RcDomWrapper { value: dom.document })
  }
}

fn symbol(mrb: &mrusty::MrubyType, str: &'static str) -> Value {
  Value::new(mrb.clone(), unsafe { MrValue::symbol_lit(mrb.borrow().mrb, str) })
}

const DEFAULT_PARSER_OPTS: ParseOpts = ParseOpts {
  tokenizer: html5ever::tokenizer::TokenizerOpts {
    exact_errors: true,
    discard_bom: true,
    profile: false,
    initial_state: None,
    last_start_tag_name: None,
  },
  tree_builder: html5ever::tree_builder::TreeBuilderOpts {
    exact_errors: true,
    scripting_enabled: false,
    iframe_srcdoc: false,
    ignore_missing_rules: false,
    drop_doctype: false,
    quirks_mode: html5ever::tree_builder::interface::QuirksMode::NoQuirks,
  },
};

mrusty_class!(RcDomWrapper, "RcDom", {
  def!("serialize", |mrb, slf: (&RcDomWrapper)| {
    let mut result: Vec<u8> = vec![];
    match serialize(&mut result, &slf.value, std::default::Default::default()) {
      Ok(()) => mrb.string(std::str::from_utf8(&result).unwrap()),
      Err(_) => panic!(),
    }
  });

  def!("children", |mrb, slf: (&RcDomWrapper)| {
    let ary: Vec<Value> = slf.value.borrow().children.iter().map(
      |v| mrb.obj(RcDomWrapper { value: v.clone() })).collect();
    mrb.array(ary)
  });

  def!("parent", |mrb, slf: (&RcDomWrapper)| {
    let parent = slf.value.borrow().parent.clone();
    match parent {
      None => mrb.nil(),
      Some(x) => {
        let weak = x.upgrade().unwrap();
        mrb.obj(RcDomWrapper {
          value: unsafe { std::mem::transmute::<_, Handle>(weak) } })
      },
    }
  });

  def!("type", |mrb, slf: (&RcDomWrapper)| {
    let sym = match slf.value.borrow().node {
      NodeEnum::Document => "document",
      NodeEnum::Doctype(_, _, _) => "doctype",
      NodeEnum::Text(_) => "text",
      NodeEnum::Comment(_) => "comment",
      NodeEnum::Element(_, _, _) => "element",
    };
    symbol(&mrb, sym)
  });

  def!("doctype", |mrb, slf: (&RcDomWrapper)| {
    let ref node = slf.value.borrow().node;
    match *node {
      NodeEnum::Doctype(ref name, ref public, ref system) =>
        mrb.array(vec![mrb.string(&*name), mrb.string(&*public), mrb.string(&*system)]),
      _ => mrb.nil(),
    }
  });

  def!("text", |mrb, slf: (&RcDomWrapper)| {
    let ref node = slf.value.borrow().node;
    match *node {
      NodeEnum::Text(ref txt) => mrb.string(&*txt),
      _ => mrb.nil(),
    }
  });

  def!("comment", |mrb, slf: (&RcDomWrapper)| {
    let ref node = slf.value.borrow().node;
    match *node {
      NodeEnum::Comment(ref cmt) => mrb.string(&*cmt),
      _ => mrb.nil(),
    }
  });

  def!("namespace", |mrb, slf: (&RcDomWrapper)| {
    let ref node = slf.value.borrow().node;
    match *node {
      NodeEnum::Element(ref name, _, _) => mrb.string(&*name.ns),
      _ => mrb.nil(),
    }
  });

  def!("name", |mrb, slf: (&RcDomWrapper)| {
    let ref node = slf.value.borrow().node;
    match *node {
      NodeEnum::Element(ref name, _, _) => mrb.string(&*name.local),
      _ => mrb.nil(),
    }
  });

  def!("element_type", |mrb, slf: (&RcDomWrapper)| {
    let ref node = slf.value.borrow().node;
    match *node {
      NodeEnum::Element(_, ref t, _) => match t {
        &ElementEnum::Normal => mrb.array(vec![symbol(&mrb, "normal")]),
        &ElementEnum::Script(l) => mrb.array(vec![symbol(&mrb, "script"), mrb.bool(l)]),
        &ElementEnum::Template(ref t) =>
          mrb.array(vec![symbol(&mrb, "template"), mrb.obj(RcDomWrapper { value: t.clone() })]),
        &ElementEnum::AnnotationXml(a) => mrb.array(vec![symbol(&mrb, "annotation_xml"), mrb.bool(a)]),
      },
      _ => mrb.nil(),
    }
  });

  def!("attributes", |mrb, slf: (&RcDomWrapper)| {
    let ref node = slf.value.borrow().node;
    match *node {
      NodeEnum::Element(_, _, ref attrs) =>
        mrb.hash(attrs.iter().map(|v| (mrb.string(&*v.name.local), mrb.string(&*v.value)))),
      _ => mrb.nil(),
    }
  });

  def!("attribute_namespaces", |mrb, slf: (&RcDomWrapper)| {
    let ref node = slf.value.borrow().node;
    match *node {
      NodeEnum::Element(_, _, ref attrs) =>
        mrb.hash(attrs.iter().map(|v| (mrb.string(&*v.name.local), mrb.string(&*v.name.ns)))),
      _ => mrb.nil(),
    }
  });

  def_self!("parse_document", |mrb, _slf: Class, str: (&str)| {
    let dom = parse_document(RcDom::default(), DEFAULT_PARSER_OPTS).one(str);
    check_parse_error(mrb, str, dom)
  });

  def_self!("parse_fragment", |mrb, _slf: Class, str: (&str)| {
    let dom = parse_fragment(
      RcDom::default(), DEFAULT_PARSER_OPTS, qualname!(html, "body"), vec![]).one(str);
    check_parse_error(mrb, str, dom)
  });
});

mrbgem_entry_fn!(mrb_mruby_html5ever_gem_init |mrb| {
  RcDomWrapper::require(mrb.clone());
});

mrbgem_entry_fn!(mrb_mruby_html5ever_gem_final |mrb| {
});
