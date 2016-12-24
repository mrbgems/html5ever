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
