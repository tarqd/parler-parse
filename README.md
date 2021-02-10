# parler-parse

Parler HTML goes in (stdin), structured JSON comes out (stdout)

Might be useful for feeding into elasticsearch or cross-referencing with the video/images dump. 

# Usage

You will need a rust compiler (easiest way is via [rustup](https://rustup.rs/)) to build from source. After that run the following commands in your terminal:

```
# clone the repo
git clone https://github.com/ilsken/parler-parse.git && cd parler-parse

# run the example
https://github.com/ilsken/parler-parse.git
cargo run < examples/echo--parent-no-comment.html

```

# Where do I get the archives?

This project was developed against the "partial parler post text" archive that available from Distributed Denial of Secrets. 

## Currently parses:

- OG Meta
- Posts + Echos 
	- Author (username + name  + avatar + badge)
	- Body
	- Media Attachments (Url, Title, Excerpt, Type, ID (numeric and base62/hex encoded))
- Comments + Replies + Engagements
- Metrics (impressions, echoes, comment count, etc)
- All mentioned usernames in the post 
- Profile pages + all posts
- Estimated timestamp offset (3 days ago -> - 3 days in seconds)


## Roadmap

- ✅ Bug: Author field will be null if a user just echoe'd a post (only has the author of the echoed post). We can populate it with the og meta title field
- ✅ Multi-threaded, recursive directory processing (crossbeam + rayon)
- ✅ Allow bulk / multi-threaded processing for all files in a directory for quickly importing into elastic/mellisearch/tantivy
- [TODO]  Add file metadata (create/modified date/path)
- [TODO]  WARC support + metadata
- [TODO]  Fix up timestamps based on metadata 




## Example output

```json
{
  "opengraph_meta": {
    "title": "@AnthonyDaubs - AnthonyDaubs -",
    "owner": {
      "name": "AnthonyDaubs",
      "username": "@AnthonyDaubs"
    },
    "url": "/post/8c36602d9568482dacfc55d9b63d5a07",
    "image_url": "https://images.parler.com/af00acf47ba74651998fb9676aabd117_256"
  },
  "posts": [
    {
      "echo_by": null,
      "cards": [
        {
          "kind": "Post",
          "author": {
            "name": "AnthonyDaubs",
            "username": "@AnthonyDaubs",
            "avatar": {
              "url_raw": "https://images.parler.com/af00acf47ba74651998fb9676aabd117_256",
              "url": "https://images.parler.com/af00acf47ba74651998fb9676aabd117_256",
              "host": "images.parler.com",
              "is_external": false,
              "id": "af00acf47ba74651998fb9676aabd117"
            }
          },
          "rel_ts": "2 days ago",
          "approx_ts_offset": -172800,
          "body": "",
          "impression_count": 3,
          "is_sensitive_content": true,
          "media_items": [
            {
              "kind": "Video",
              "title": "",
              "link": {
                "label": "https://video.parler.com/Q2/s5/Q2s5oVN1pfgk_small.mp4",
                "url_raw": "https://video.parler.com/Q2/s5/Q2s5oVN1pfgk_small.mp4",
                "url": "https://video.parler.com/Q2/s5/Q2s5oVN1pfgk_small.mp4",
                "host": "video.parler.com",
                "is_external": false,
                "id": "Q2s5oVN1pfgk",
                "id_b62_dec": 1355361448748163000000
              },
              "excerpt": "",
              "source": {
                "label": "",
                "url_raw": "https://video.parler.com/Q2/s5/Q2s5oVN1pfgk_small.mp4",
                "url": "https://video.parler.com/Q2/s5/Q2s5oVN1pfgk_small.mp4",
                "host": "video.parler.com",
                "is_external": false,
                "id": "Q2s5oVN1pfgk",
                "id_b62_dec": 1355361448748163000000
              },
              "numeric_id": null
            }
          ]
        }
      ],
      "comments": [],
      "post_id": null,
      "mentions": [],
      "engagements": {
        "comment_count": 0,
        "echo_count": 0,
        "upvote_count": 0
      }
    }
  ]
}

```


# License

MIT licensed, feel free to use it. If you want to use it for research, I'd love to hear about it and help if I can. Shoot me an email or message me on twitter (@chris_tarquini)
