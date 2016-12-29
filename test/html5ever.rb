assert 'RcDom' do
  assert_equal Class, RcDom.class
end

assert 'RcDom.parse_document' do
  assert_equal RcDom, RcDom.parse_document("<!doctype html>").class
end

assert 'RcDom#serialize' do
  assert_equal "<!DOCTYPE html><html><head></head><body></body></html>",
               RcDom.parse_document("<!doctype html>").serialize
end

assert 'RcDom.parse_fragment' do
  str = '<base foo="&amp;">'
  assert_equal "<html>#{str}</html>", RcDom.parse_fragment(str).serialize
end

assert 'RcDom#children' do
  str = '<table><tbody><tr><td>test</td></tr></tbody></table>'
  assert_equal '<tbody><tr><td>test</td></tr></tbody>', RcDom.parse_fragment(str).children[0].children[0].serialize
end

assert 'RcDom#parent' do
  str = '<table><tr><td>test</td></tr></table>'
  parsed = RcDom.parse_fragment str
  assert_equal parsed.serialize, parsed.children[0].parent.serialize
end

assert 'RcDom#type' do
  parsed = RcDom.parse_document("<!doctype html>")
  assert_equal :document, parsed.type
  assert_equal :element, parsed.children[1].type
end

assert 'RcDom#doctype' do
  parsed = RcDom.parse_document("<!doctype html>")
  assert_equal ['html', '', ''], parsed.children[0].doctype
end

assert 'RcDom#text' do
  parsed = RcDom.parse_fragment('<div>text</div>text test').children[0].children[0].children[0]
  assert_equal 'text', parsed.text
  assert_equal :text, parsed.type
end

assert 'RcDom#comment' do
  parsed = RcDom.parse_fragment('<!--comment-->').children[0].children[0]
  assert_equal 'comment', parsed.comment
  assert_equal :comment, parsed.type
end

assert 'RcDom#namespace' do
  parsed = RcDom.parse_fragment('<div>text</div>text test').children[0]
  assert_equal :element, parsed.type
  assert_equal 'http://www.w3.org/1999/xhtml', parsed.namespace
end

assert 'RcDom#name' do
  parsed = RcDom.parse_fragment('<div>text</div>text test').children[0]
  assert_equal :element, parsed.type
  assert_equal 'html', parsed.name
  assert_equal 'div', parsed.children[0].name
end

assert 'RcDom#element_type' do
  parsed = RcDom.parse_fragment('<div>text</div>text test').children[0]
  assert_equal :element, parsed.type
  assert_equal [:normal], parsed.element_type
end

assert 'RcDom#attributes' do
  parsed = RcDom.parse_fragment('<div data-id="test" data-value="8">htest_txt</div>').children[0].children[0]
  assert_equal :element, parsed.type
  assert_equal 'div', parsed.name
  assert_equal({ 'data-id' => 'test', 'data-value' => '8' }, parsed.attributes)
end

assert 'RcDom#attributes_namespaces' do
  parsed = RcDom.parse_fragment('<div data-id="test" data-value="8">test_txt</div>').children[0].children[0]
  assert_equal :element, parsed.type
  assert_equal 'div', parsed.name
  assert_equal({ 'data-id' => '', 'data-value' => '' }, parsed.attribute_namespaces)
end
