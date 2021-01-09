This crate is an experiment to run actix-web and Paperclip on AWS Lambda using `netlify_lambda_http` (a fork of the official AWS runtime that's being more actively developed).

## Credits
1. Inspiration for the custom handler was taken from [warp_lambda](https://github.com/aslamplr/warp_lambda).
2. [This GitHub issue](https://github.com/actix/actix-web/issues/768) pointed me in the right direction for processing actix-web requests directly without running a web server.
3. I used the [examples in Paperclip's documentation](https://paperclip.waffles.space/actix-plugin.html) to add OpenAPI 2 documentation.