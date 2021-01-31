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
	- Author (username + name)
	- Body
	- Media Attachments (Url, Title, Excerpt, Type)
- Comments
- Metrics (impressions, echoes, comment count, etc)
- All mentioned usernames in the post 


## TODO

- [DONE] Bug: Author field will be null if a user just echoe'd a post (only has the author of the echoed post). We can populate it with the og meta title field
- [] Allow bulk / multi-threaded processing for all files in a directory for quickly importing into elastic/mellisearch/tantivy
- [] Allow automatically downloading requested videos/images from the distributed denial of secrets s3 (will require s3 api key and will cost money)




## Example output

```json
{
  "opengraph_meta": {
    "title": "@tacticallyefficient - tacticallyefficient - @RudyG \n@JennaEllisEsq \n@SidneyPowell",
    "url": "/post/b9c94be7d211411eae18d83533a68638",
    "image_url": "https://images.parler.com/d996a859e8644d77bb30f5a4de519b48_256"
  },
  "post": {
    "cards": [
      {
        "kind": "EchoParent",
        "author": {
          "name": "Justice Will Prevail 🌎",
          "username": "@JusticeByLight"
        },
        "body": "",
        "impression_count": 2330,
        "media_container": {
          "sensitive_id": null,
          "media_items": [
            {
              "meta": {
                "title": null,
                "link": {
                  "kind": "Image",
                  "label": "Image",
                  "location": "https://api.parler.com/l/9zQbh"
                },
                "excerpt": null
              }
            }
          ]
        }
      },
      {
        "kind": "Post",
        "author": {
          "name": "truthseeker",
          "username": "@tacticallyefficient"
        },
        "body": "@RudyG@JennaEllisEsq@SidneyPowell",
        "impression_count": 58,
        "media_container": {
          "sensitive_id": null,
          "media_items": []
        }
      }
    ],
    "comments": [
      {
        "author": {
          "name": "SusanC.H.",
          "username": "@SusanH08"
        },
        "body": "Doesn’t matter!! Take a listen. It’s long. But it’s good!! Pray for this woman. She is risking her life to save our asses. Let people know we have silent warriors! 🔥🔥🇺🇸🇺🇸"
      }
    ],
    "mentions": [
      "@RudyG",
      "@JennaEllisEsq",
      "@SidneyPowell"
    ],
    "engagements": {
      "comment_count": 7,
      "echo_count": 37,
      "upvote_count": 44
    }
  }
}
```


# License

MIT licensed, feel free to use it. If you want to use it for research, I'd love to hear about it and help if I can. Shoot me an email or message me on twitter (@chris_tarquini)
