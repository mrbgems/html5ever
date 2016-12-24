MRuby::Gem::Specification.new 'mruby-html5ever' do |spec|
  spec.license = 'MIT'
  spec.author = 'Takeshi Watanabe'
  spec.summary = 'mruby binding of html5ever'

  add_cargo_dependency 'html5ever', '*'
  add_cargo_dependency 'html5ever-atoms', '*'
  add_cargo_dependency 'tendril', '*'
  add_cargo_dependency 'selectors', '*'
end
