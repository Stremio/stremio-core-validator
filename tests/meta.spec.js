const validator = require('../stremio_core_validator');

describe('meta', () => {
    it('MetaItemPreview', async () => {
        const meta = validator.meta_item_preview({
            id: 'id',
            type: 'type',
            name: 'name'
        });

        expect(meta).toEqual({
            id: 'id',
            type: 'type',
            name: 'name',
            description: null,
            logo: null,
            poster: null,
            background: null,
            posterShape: 'poster',
            releaseInfo: null,
            released: null,
            runtime: null,
            trailerStreams: [],
            links: [],
            behaviorHints: {
                defaultVideoId: null,
                featuredVideoId: null,
                hasScheduledVideos: false
            }
        });
    });
});
