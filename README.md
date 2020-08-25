# Stremio Core Validator

Stremio JSON Schema Validator for Node.js

## Build

```
npm run build
```

## Example

### MetaPreview validation

`validator.meta_preview` returns null if the argument does not match the schema.
Additional properties that are not part of the MetaPreview schema got removed.

```javascript
const validator = require('@stremio/stremio-core-validator');
const meta = validator.meta_preview({
    id: 'id',
    type: 'type',
    name: 'name'
});
```
