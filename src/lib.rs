#[macro_use] extern crate mrusty;
extern crate tendril;
extern crate html5ever;
#[macro_use] extern crate html5ever_atoms;
extern crate selectors;

use html5ever::ParseOpts;
use html5ever::{parse_fragment, parse_document, serialize};
use html5ever::rcdom::RcDom;
use html5ever::rcdom::Handle;

use mrusty::{MrubyFile, MrubyImpl, Value};

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
