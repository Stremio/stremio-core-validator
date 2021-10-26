const validator = require('../stremio_core_validator');

describe('meta', () => {
    it('MetaItemPreview', async () => {
        const meta = validator.meta_item_preview(JSON.stringify({
            id: 'id',
            type: 'type',
            name: 'name'
        }));

        expect(meta).toEqual({
            id: 'id',
            type: 'type',
            name: 'name',
            description: null,
            logo: null,
            poster: null,
            posterShape: 'poster',
            releaseInfo: null,
            released: null,
            runtime: null,
            trailerStreams: [],
            behaviorHints: {
                defaultVideoId: null,
                featuredVideoId: null,
                hasScheduledVideos: false
            }
        });
    });
});
