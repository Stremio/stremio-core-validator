# Stremio Core Validator

Stremio JSON Schema Validator for Node.js using stremio-core types

Additional properties that are not part of the schema got removed. Validation functions returns null if the argument does not match the schema.

## Build

```
npm run build
```

## Supported types
- Manifest
- MetaPreview
- MetaDetail
- Stream

## Example

### MetaPreview validation

```javascript
const validator = require('@stremio/stremio-core-validator');
const meta = validator.meta_preview({
    id: 'id',
    type: 'type',
    name: 'name'
});
```
