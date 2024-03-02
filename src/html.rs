// Module to add HTML headers

#[allow(dead_code)]
pub fn add_headers(title: &str, headline: &str, tags: &str, author: &str, body: &str) -> String {
    format!(
        "<!DOCTYPE html>\n\
        <html lang=\"en\">\n\
        <head>\n\
            <meta charset=\"UTF-8\">\n\
            <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n\
            <meta name=\"description\" content=\"{}\">\n\
            <meta name=\"author\" content=\"{}\">\n\
            <meta name=\"keywords\" content=\"{}\">\n\
            <link rel=\"icon\" type=\"image/png\" href=\"https://cdn-icons-png.flaticon.com/512/3135/3135715.png\">\n\
            <title>{}</title>\n\
            <style>\n\
            .center{{text-align: center;}}\n\
            </style>\n\
        </head>\n\
        \
        <body style=\"background-color:black; color:cornsilk;\">\n\
        <header>\n\
            <div class=\"center\">\n\
                <h1>{}</h1>\n\
                <hr>\n\
            </div>\n\
        </header>\n\
        <main>\n\
            <h1>{}</h1>\n\
            {}\
        </main>\n\
        <footer>\n\
        </footer>\n\
        </body>\n\
        </html>",
        headline, author, tags, title, title, headline, body
    )
}
