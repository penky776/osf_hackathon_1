const csrfTokenUrl = '/get-csrf-token';

fetch(csrfTokenUrl)
    .then((response) => response.text())
    .then((token) => {
        {
            sendCSRFHeader("addpost", token);
            sendCSRFHeader("deletepost", token);

            var hiddenInputs = document.getElementsByClassName("csrf_token");
            for (var i = 0; i < hiddenInputs.length; i++) {
                hiddenInputs[i].value = token;
            }
        }
    })
    .catch((error) => {
        console.error('Error fetching CSRF token:', error);
    });

// TODO
function sendCSRFHeader(formId, token) {
    const form = document.getElementById(formId);
    form.addEventListener("submit", function (event) {
        event.preventDefault();
        const xhr = new XMLHttpRequest();
        const currentUrl = window.location.href;
        const url = currentUrl + formId;

        xhr.open("POST", url, true);
        xhr.setRequestHeader("X-CSRF-TOKEN", token);
        xhr.setRequestHeader("Content-Type", "application/x-www-form-urlencoded");
        xhr.onreadystatechange = function () {
            if (xhr.readyState === XMLHttpRequest.DONE) {
                if (xhr.status === 200) {
                    console.log("Request completed!");
                    console.log(xhr.responseText);
                } else {
                    console.error("Request failed with status:", xhr.status);
                }
            }
        };

        const originalformData = new FormData(form);

        const data = {};
        originalformData.forEach((value, key) => {
            data[key] = value;
        });

        const formData = new URLSearchParams();

        for (const key in data) {
            formData.append(key, data[key]);
        }

        const formBody = formData.toString();

        xhr.send(formBody); // Send the request
    });
}
