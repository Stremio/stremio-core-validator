const validator = require('../stremio_core_validator');

describe('stream', () => {
    it('Stream', async () => {
        const stream = validator.stream({
            name: 'name',
            title: 'title',
            externalUrl: 'https://example.com/',
            androidTvUrl: 'intent://example.com/',
            tizenUrl: '{\"id\":\"com.example\",\"action_data\":\"https://example.com\"}',
            webosUrl: '{\"id\":\"com.example\",\"params\":\"{}\"}'
        });

        expect(stream).toEqual({
            name: 'name',
            description: 'title',
            externalUrl: 'https://example.com/',
            androidTvUrl: 'intent://example.com/',
            tizenUrl: '{\"id\":\"com.example\",\"action_data\":\"https://example.com\"}',
            webosUrl: '{\"id\":\"com.example\",\"params\":\"{}\"}'
        });
    });
});

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
