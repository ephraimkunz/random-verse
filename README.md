# random-verse
Simple API endpoint to get a random scripture verse each day. Day here is defined as a calendar day on the west coast (Pacific Time Zone).

* To use, hit https://random-verse.shuttleapp.rs. You'll receive JSON like this:

```json
{
  "volume_title": "Doctrine and Covenants",
  "book_title": "Doctrine and Covenants",
  "book_short_title": "D&C",
  "chapter_number": 132,
  "verse_number": 14,
  "verse_title": "Doctrine and Covenants 132:14",
  "verse_short_title": "D&C 132:14",
  "scripture_text": "For whatsoever things remain are by me; and whatsoever things are not by me shall be shaken and destroyed."
}
```

* This project uses [Shuttle](https://www.shuttle.rs) to deploy and serve content. Run `cargo shuttle deploy --allow-dirty` to deploy a new version.
* Book of Mormon and KJV bible text comes from the [LDS Documentation Project](https://scriptures.nephi.org/start).
