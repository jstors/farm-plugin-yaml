# farm-plugin-yaml

This plugin provides a YAML parser for FARM.

## Usage

```bash
pnpm install -D @jstors/farm-plugin-yaml
```


### Configuration
```ts

export default defineConfig({
  plugins: [
      '@jstors/farm-plugin-yaml',
  ]
});

```
### Template

```tsx
import yamlFile from './base.yaml'

console.log(yamlFile)
```

## Transform Code
```yaml
languages:
  - Ruby
  - Perl
  - Python 
websites:
  YAML: yaml.org 
  Ruby: ruby-lang.org 
  Python: python.org 
  Perl: use.perl.org
number: 1
is: true
c: 
```

```js
module.exports = {"websites":{"YAML":"yaml.org","Ruby":"ruby-lang.org","Python":"python.org","Perl":"use.perl.org"},"is":"true","number":"1","c":"null","languages":["Ruby","Perl","Python"]}
```

