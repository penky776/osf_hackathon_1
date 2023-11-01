fetch('http://localhost:3000/static/json/posts.json', {
    method: 'GET',
    headers: {
        'Accept': 'application/json',
    },
})

    .then(postJSON => postJSON.json())
    .then(

        postJSON => {
            class Posts {
                getId() {
                    return this.post_id;
                }
                getTitle() {
                    return this.title;
                }
                getAuthor() {
                    return this.author;
                }
                getBody() {
                    return this.body;
                }
            }
            let newPost = Object.assign(new Posts(), postJSON);
            const app = document.getElementById("content");
            const p = document.createElement("p");

            p.textContent = newPost.getBody();
            app === null || app === void 0 ? void 0 : app.appendChild(p);

        }

    )

