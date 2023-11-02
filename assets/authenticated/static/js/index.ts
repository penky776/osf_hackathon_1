// typescript was used here for convenience only 
import postJSON from '../api/json/posts.json'

class Posts {
    post_id: number;
    title: string;
    author: string;
    body: string;

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
app?.appendChild(p);