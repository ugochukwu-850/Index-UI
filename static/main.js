document.addEventListener('DOMContentLoaded', function() {
    // Your existing code...

    // Example:
    // var fileArray = document.getElementById('fileArray');
    // fileArray.value = JSON.stringify({});

    document.getElementById('upload_files').addEventListener('change', function(e) {
        var files = e.target.files;
        var formData = new FormData();

        for (var i = 0; i < files.length; i++) {
            var file = files[i];
            var reader = new FileReader();

            reader.onload = (function(file, i) {
                return function(event) {
                    formData.append('files[]', file, file.name);
                    formData.append("filer[]", "game chaner");
                    alert("Added");

                    var li = document.createElement('li');
                    li.setAttribute('data-id', 'file-' + i++);

                    var input = document.createElement('input');
                    input.setAttribute('name', 'files[]');
                    input.setAttribute('id', 'fileArray');
                    input.setAttribute('type', 'hidden');

                    var div = document.createElement('div');
                    div.setAttribute('class', 'file-container');

                    var div1 = document.createElement('div');
                    div1.setAttribute('class', 'removebtn');
                    div1.setAttribute('id', 'fileremove');

                    var div2 = document.createElement('div');
                    div2.setAttribute('class', 'filename');

                    div2.appendChild(document.createTextNode(file.name));

                    li.appendChild(input);
                    li.appendChild(div);
                    li.appendChild(div1);
                    li.appendChild(div2);

                    document.getElementById('filelist').prepend(li);
                }
            })(file, i);

            reader.readAsDataURL(file);
        }

        console.log(formData.get("files[]"));
        alert(formData);
        

        // Add your fetch code here to send the formData to the server
        // Example:
         fetch('/upload', {
             method: 'POST',
             body: formData
         })
         .then(response => response.json())
         .then(data => console.log(data))
         .catch(error => console.error('Error:', error));
    });

    document.getElementById('filelist').addEventListener('click', function(e) {
        var target = e.target;
        if (target.classList.contains('removebtn')) {
            target.closest('li').remove();
            // Call your removeItem function if needed
        }
    });

    // Your existing code...
});


// returns 
function tdq_type(e) {
    let x = e.isChecked();
}