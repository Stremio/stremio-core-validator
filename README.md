# Stremio Core Validator

![Build](https://github.com/stremio/stremio-core-validator/workflows/Build/badge.svg)

Stremio JSON Schema Validator for Node.js using stremio-core types

Additional properties that are not part of the schema got removed. Validation functions throws error if the argument does not match the schema.

## Build

```bash
npm run build
```

## Supported types

- Manifest
- ManifestPreview
- Descriptor
- DescriptorPreview
- MetaItem
- MetaItemPreview
- Stream
- Subtitles
- Video
- ResourceResponse

## Example

### MetaItemPreview validation

meta refers to javascript object with valid MetaItemPreview schema

```javascript
const validator = require('@stremio/stremio-core-validator');
const meta = validator.meta_item_preview({
    id: 'id',
    type: 'type',
    name: 'name'
});
```

### Cathing errors

error refers to Error object with message describing which field is invalid

```javascript
const validator = require('@stremio/stremio-core-validator');
try {
    const meta = validator.meta_item_preview({
        id: 'id',
        // type: 'type',
        name: 'name'
    });
} catch (error) {
    console.log(error.message);
}
```
