import { nextTestSetup } from 'e2e-utils'

describe('ppr-metadata-streaming', () => {
  const { next } = nextTestSetup({
    files: __dirname,
  })

  // No dynamic APIs used in metadata
  describe('static metadata', () => {
    it('should generate metadata in body when page is fully static', async () => {
      const $ = await next.render$('/fully-static')
      expect($(`body title`).text()).toBe('fully static')
    })

    it('should insert metadata in body when page is dynamic page content', async () => {
      const $ = await next.render$('/dynamic-page')
      expect($(`body title`).text()).toBe('dynamic page')
    })
  })

  // Dynamic APIs used in metadata, metadata should be suspended and inserted into body
  describe('dynamic metadata', () => {
    it('should generate metadata in head when page is fully dynamic', async () => {
      const $ = await next.render$('/fully-dynamic')
      expect($('body title').text()).toBe('fully dynamic')
    })

    it('should generate metadata in head when page content is static', async () => {
      const $ = await next.render$('/dynamic-metadata')
      expect($('body title').text()).toBe('dynamic metadata')
    })
  })

  describe('partial shell', () => {
    it('should insert metadata into body with dynamic metadata and wrapped under layout Suspense boundary', async () => {
      const $ = await next.render$('/dynamic-metadata/partial')
      expect($('body title').text()).toBe('dynamic-metadata - partial')

      const browser = await next.browser('/dynamic-metadata/partial')
      expect(await browser.waitForElementByCss('title').text()).toBe(
        'dynamic-metadata - partial'
      )
    })

    it('should insert metadata into body with dynamic metadata and dynamic page wrapped under layout Suspense boundary', async () => {
      const $ = await next.render$('/dynamic-page/partial')
      expect($('body title').text()).toBe('dynamic-page - partial')

      const browser = await next.browser('/dynamic-page/partial')
      expect(await browser.waitForElementByCss('title').text()).toBe(
        'dynamic-page - partial'
      )
    })
  })
})
