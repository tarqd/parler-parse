# parler-parse

Parler HTML goes in (stdin), structured JSON comes out (stdout)

Might be useful for feeding into elasticsearch or cross-referencing with the video/images dump. 

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

- [] Bug: Author field will be null if a user just echoe'd a post (only has the author of the echoed post). We can populate it with the og meta title field
- [] Allow bulk / multi-threaded processing for all files in a directory for quickly importing into elastic/mellisearch/tantivy
- [] Allow automatically downloading requested videos/images from the distributed denial of secrets s3 (will require s3 api key and will cost money)




