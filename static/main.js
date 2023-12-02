//import JSZip from 'https://cdnjs.cloudflare.com/ajax/libs/jszip/3.7.1/jszip.min.js';

/*
var queryType = 0;
var formData = new FormData();
var activeProcess = null;
var Files = [];


document.addEventListener('DOMContentLoaded', function () {
    const SearchButton = document.querySelector("#startsearch");
    const downloadButton = document.querySelector("#downloadButton");
    const linkDownloadButton = document.querySelector("#downloadlink");

    downloadButton.setAttribute("disabled", "");

    // Example:
    // var fileArray = document.getElementById('fileArray');
    // fileArray.value = JSON.stringify({});
    document.querySelectorAll('.upload-btn-files').forEach(handler => {
        handler.addEventListener('change', function (e) {
            handleUploadFileS(e);
        });
    });


    //only when a file has been addded allow the search

    SearchButton.addEventListener("click", (e) => {
        // if already in progres of another query
        if (!activeProcess == null) {
            alert("There is already a running process. Please wait");
            return;
        }

        if (Files.length == 0) {
            alert("Please add some files");
            return;
        }

        // get the queries
        let queries = parseQueries();
        if (queries == null) {
            alert("Please add some query data!!!");
            return;
        }

        // batch the amount of files
        let files_batch = batchFile()

        if (files_batch == null) {
            alert("Batch Exceeding limits of a single file!! \n Try zipping the files");
            return;
        }
        console.log(files_batch);

        let query_create_process = {
            "CreateProcess": [files_batch.length - 1, queries]
        }
        console.log(query_create_process);

        formData.append("action", JSON.stringify(query_create_process));
        formData = add_files_to_form(formData, files_batch[0]);
        //make the request 

        //set to searching
        SearchButton.textContent = "Searching";
        SearchButton.setAttribute("disabled", "");
        downloadButton.setAttribute("disabled", "");
        linkDownloadButton.setAttribute("hidden", "");
        // return;

        fetch('/upload', {
            method: 'POST',
            body: formData
        })
            .then(response => response.json())
            .then(creationData => {
                // if successfull create the batch in the front end
                activeProcess = {
                    "batchCount": files_batch.length - 1,
                    "isComplete": false,
                    "totalFiles": Files.length,
                    "completedBatches": 0,
                    "proc_id": creationData["proc_id"]
                };

                //alert("The process has been created")
                // also update active process dom and the add the bacth Id as though
                // document.querySelector("#dom").textContent = 1;
                // document.querySelector("#dom").textContent = `Process Id: ${data["proc_id"]}`;

                // now start the batch sequence if there is 
                console.log("name", creationData);
                //var data = data;
                if (activeProcess.batchCount == 0) {
                    // alert("completed One Time Process");
                    // change the search button to download link
                    downloadButton.removeAttribute("disabled");
                    linkDownloadButton.href = `/download/${creationData["proc_id"]}`;
                    linkDownloadButton.removeAttribute("hidden");
                    linkDownloadButton.click();
                    activeProcess.isComplete = true;
                    console.log(activeProcess);
                    activeProcess = null;
                    SearchButton.textContent = "Search";
                    SearchButton.removeAttribute("disabled", "");

                    // disable the download button on timeout
                    setTimeout(() => {
                        downloadButton.setAttribute("disabled", "");
                        linkDownloadButton.href = ``;
                        linkDownloadButton.setAttribute("hidden", "");
                    }, 1000 * 60 * 5);
                }
                else {
                    // start a stream of search
                    console.log("Attempting to start for sub stream");
                    for (var index = 1; index < files_batch.length; index++) {

                        const files = files_batch[index];
                        let newformData = new FormData();
                        newformData = add_files_to_form(newformData, files)
                        newformData.append("action", JSON.stringify({
                            "Stream": [
                                creationData["proc_id"], queries, index
                            ]
                        }))

                        // make the request
                        console.log("About to start for index " + index);

                        fetch('/upload', {
                            method: 'POST',
                            body: newformData
                        })
                            .then(sec_res => sec_res.json())
                            .then(data => {
                                console.log(data, index);
                                if (data.status == 200 || true) {
                                    activeProcess.completedBatches += 1;
                                    activeProcess.isComplete = activeProcess.completedBatches == activeProcess.batchCount ? true : false;

                                    // do the logs 
                                    console.log(activeProcess, data);

                                    if (activeProcess.isComplete == true) {

                                        downloadButton.removeAttribute("disabled");
                                        linkDownloadButton.href = `/download/${data["proc_id"]}`;
                                        linkDownloadButton.removeAttribute("hidden");
                                        linkDownloadButton.click();
                                        console.log(activeProcess);
                                        SearchButton.textContent = "Search";
                                        SearchButton.removeAttribute("disabled", "");
                                        activeProcess = null;
                                    };

                                };
                            })
                            .catch(e => {
                                console.error("Failed to request batch " + index, e);
                                return;
                            });

                    }
                }
            })
            .catch(error => {
                alert("An errored occured whilst opening this batch");
                console.error('Error:', error);
                return;
                //location.reload();
            });


    })

});


function handleUploadFileS(e) {
    var files = e.target.files;
    var filelist = document.getElementById('filelist');
    var total_size = 0;
    filelist.innerHTML = "";
    for (var i = 0; i < files.length; i++) {
        var file = files[i];
        var reader = new FileReader();
        //formData.append('files[]', file);
        total_size = total_size + file.size / 1000;

        reader.onload = (function (file, i) {
            return function () {
                //formData.append('files[]', file);

                var li = document.createElement('li');
                li.setAttribute('data-id', 'file-' + i++);

                var input = document.createElement('input');
                input.setAttribute('name', 'files[]');
                input.setAttribute('id', 'fileArray');
                input.setAttribute('type', 'hidden');

                var div = document.createElement('div');
                div.setAttribute('class', 'file-container');


                var div2 = document.createElement('div');
                div2.setAttribute('class', 'filename');


                div2.appendChild(document.createTextNode(file.name));

                li.appendChild(input);
                li.appendChild(div);
                li.appendChild(div2);


                filelist.prepend(li)
            }
        })(file, i);

        reader.readAsDataURL(file);
    }

    // updated the total file count
    document.querySelector("#files_detected").textContent = `${files.length}`;

    total_size < 1000 ? document.querySelector("#file_size").textContent = `${total_size.toFixed(2)}Kb` : document.querySelector("#file_size").textContent = `${(total_size / 1000).toFixed(2)} Mb`;

    // show files added
    Files = files;
}


function add_data_tag(e) {
    console.log("Added");
    let new_tag = document.createElement("input");
    new_tag.setAttribute("class", "queryvaluetag");
    new_tag.setAttribute("placeholder", "Set data")
    new_tag.setAttribute("type", "text");
    e.target.parentElement.prepend(new_tag);
}

function removeQueryCard(e) {
    e.target.parentElement.remove();
    let query_dash_count = document.querySelector("#queries_detected");

    let query_count = parseInt(query_dash_count.textContent);
    query_dash_count.textContent = `${query_count - 1}`
}

function createQueryCard(e) {
    let card = document.createElement("div");
    card.setAttribute("class", "query_card");

    card.innerHTML = queryType == 0 ? `<button onclick="removeQueryCard(event);" class="removequery">-</button>
    <input type="text" placeholder="Search Title" name="query_title1" id="query_title_1" class="query_title">
    <div class="tags">
      <input type="text" placeholder="Data Tag" class="queryvaluetag">
      <input type="button" value="+" id="csqt" onclick="add_data_tag(event);">
    </div>
  </div>` : `<button onclick="removeQueryCard(event);" class="removequery">-</button>
  <input type="text" placeholder="Search Title" name="query_title1" id="query_title_1" class="query_title">`;
    document.querySelector(".queries").append(card);
    let query_dash_count = document.querySelector("#queries_detected");
    let query_count = parseInt(query_dash_count.textContent);
    query_dash_count.textContent = `${query_count + 1}`
}

function switch_query_type(e) {

    if (e.target.classList.contains("active")) {
        return;
    }
    let queries_cont = document.querySelectorAll(".query_card");
    document.querySelectorAll(".top h4").forEach(f => {
        f.classList.remove("active");
    });
    e.target.classList.add("active");
    queries_cont.forEach(e => {
        e.remove()
    });
    let query_dash_count = document.querySelector("#queries_detected");
    query_dash_count.textContent = `0`;
    if (e.target.textContent == "Title and Data Query") {
        queryType = 0;
    }
    else {
        queryType = 1;
    }
}

function parseQueries() {
    var query = {};
    var total_queries = 0;

    if (queryType == 0) {
        query["TitleData"] = {};
        // T+D query
        document.querySelectorAll(".query_card").forEach(e => {
            var title;
            var data = [];
            e.childNodes.forEach(c => {
                // get the title
                if (c.classList && c.classList.contains("query_title") && c.value.length > 0) {
                    title = c.value.trim();
                    //console.log(title);
                }
                else if (c.classList && c.classList.contains("tags")) {
                    c.childNodes.forEach(d => {
                        if (d.classList && d.classList.contains("queryvaluetag") && d.value.length > 0) {
                            data.push(d.value.trim());
                        }
                    })
                }
            })
            //console.log(title);
            if (title && data.length > 0) {
                total_queries += 1;
                query["TitleData"][title] = data;
                //query["TitleData"].push(datum);
            }
            //console.log(query.TitleData[0]);

        });
    }
    else {
        let keys = [];
        // T+D query
        document.querySelectorAll(".query_card").forEach(e => {
            e.childNodes.forEach(c => {
                // get the title
                if (c.classList && c.classList.contains("query_title") && c.value.length > 0) {
                    keys.push(c.value.trim());
                    //console.log(keys);
                }

            })
        });
        if (keys.length > 0) {
            query["OnlyData"] = keys;
        }

    }
    console.log(query);
    return (query.OnlyData || total_queries > 0) ? query : null;
}



function batchFile() {
    let filelist = Files;
    var container = [];
    var current_list = [];
    var active_list_len = 0;


    for (let index = 0; index < filelist.length; index++) {
        // 1000 = 1kb ; 1000,000= 1mb; 100,000,000 = 100mb,
        if (filelist[index].size > 100000000) {
            return null;
        }

        if (active_list_len < 100000000 /*kilobyte 100mb/) {
            current_list.push(filelist[index])
            active_list_len = active_list_len + filelist[index].size;

        }
        else {
            container.push(current_list);
            current_list = [];
            current_list.push(filelist[index])
            active_list_len = 0;
        }

    }
    container.push(current_list);

    return container;
}

function test() {

    let file = { "size": 2000000 };
    var filelist = new Array(10000).fill(file);
    console.log(filelist.length);
    let batch = batchFile(filelist);
    console.log(batch);
}

function add_files_to_form(form, files) {
    for (let index = 0; index < files.length; index++) {
        const file = files[index];

        form.append("files[]", file);
    }
    return form;
}
*/


//import JSZip from 'https://cdnjs.cloudflare.com/ajax/libs/jszip/3.7.1/jszip.min.js';

var queryType = 0;
var formData = new FormData();
var activeProcess = null;
var Files = [];


document.addEventListener('DOMContentLoaded', function () {
    const SearchButton = document.querySelector("#startsearch");
    const downloadButton = document.querySelector("#downloadButton");
    const linkDownloadButton = document.querySelector("#downloadlink");

    downloadButton.setAttribute("disabled", "");

    // Example:
    // var fileArray = document.getElementById('fileArray');
    // fileArray.value = JSON.stringify({});
    document.querySelectorAll('.upload-btn-files').forEach(handler => {
        handler.addEventListener('change', function (e) {
            handleUploadFileS(e);
        });
    });


    //only when a file has been addded allow the search

    SearchButton.addEventListener("click", (e) => {
        // if already in progres of another query
        if (!activeProcess == null) {
            alert("There is already a running process. Please wait");
            return;
        }

        if (Files.length == 0) {
            alert("Please add some files");
            return;
        }

        // get the queries
        let queries = parseQueries();
        if (queries == null) {
            alert("Please add some query data!!!");
            return;
        }

        // batch the amount of files
        let files_batch = batchFile()

        if (files_batch == null) {
            alert("Batch Exceeding limits of a single file!! \n Try zipping the files");
            return;
        }
        console.log(files_batch);

        let query_create_process = {
            "CreateProcess": [files_batch.length - 1, queries]
        }
        console.log(query_create_process);

        formData.append("action", JSON.stringify(query_create_process));
        formData = add_files_to_form(formData, files_batch[0]);
        //make the request 

        //set to searching
        SearchButton.textContent = "Searching";
        SearchButton.setAttribute("disabled", "");
        downloadButton.setAttribute("disabled", "");
        linkDownloadButton.setAttribute("hidden", "");
        // return;

        fetch('/upload', {
            method: 'POST',
            body: formData
        })
            .then(response => response.json())
            .then(creationData => {
                // if successfull create the batch in the front end
                activeProcess = {
                    "batchCount": files_batch.length - 1,
                    "isComplete": false,
                    "totalFiles": Files.length,
                    "completedBatches": 0,
                    "proc_id": creationData["proc_id"]
                };

                //alert("The process has been created")
                // also update active process dom and the add the bacth Id as though
                // document.querySelector("#dom").textContent = 1;
                // document.querySelector("#dom").textContent = `Process Id: ${data["proc_id"]}`;

                // now start the batch sequence if there is 
                console.log("name", creationData);
                //var data = data;
                if (activeProcess.batchCount == 0) {
                    // alert("completed One Time Process");
                    // change the search button to download link
                    downloadButton.removeAttribute("disabled");
                    linkDownloadButton.href = `/download/${creationData["proc_id"]}`;
                    linkDownloadButton.removeAttribute("hidden");
                    linkDownloadButton.click();
                    activeProcess.isComplete = true;
                    console.log(activeProcess);
                    activeProcess = null;
                    SearchButton.textContent = "Search";
                    SearchButton.removeAttribute("disabled", "");

                    // disable the download button on timeout
                    setTimeout(() => {
                        downloadButton.setAttribute("disabled", "");
                        linkDownloadButton.href = ``;
                        linkDownloadButton.setAttribute("hidden", "");
                    }, 1000 * 60 * 5);
                }
                else {
                    // start a stream of search
                    console.log("Attempting to start for sub stream");
                    for (var index = 1; index < files_batch.length; index++) {

                        const files = files_batch[index];
                        let newformData = new FormData();
                        newformData = add_files_to_form(newformData, files)
                        newformData.append("action", JSON.stringify({
                            "Stream": [
                                creationData["proc_id"], queries, index
                            ]
                        }))

                        // make the request
                        console.log("About to start for index " + index);

                        fetch('/upload', {
                            method: 'POST',
                            body: newformData
                        })
                            .then(sec_res => sec_res.json())
                            .then(data => {
                                console.log(data, index);
                                if (data.status == 200 || true) {
                                    activeProcess.completedBatches += 1;
                                    activeProcess.isComplete = activeProcess.completedBatches == activeProcess.batchCount ? true : false;

                                    // do the logs 
                                    console.log(activeProcess, data);

                                    if (activeProcess.isComplete == true) {

                                        downloadButton.removeAttribute("disabled");
                                        linkDownloadButton.href = `/download/${data["proc_id"]}`;
                                        linkDownloadButton.removeAttribute("hidden");
                                        linkDownloadButton.click();
                                        console.log(activeProcess);
                                        SearchButton.textContent = "Search";
                                        SearchButton.removeAttribute("disabled", "");
                                        activeProcess = null;
                                    };

                                };
                            })
                            .catch(e => {
                                console.error("Failed to request batch " + index, e);
                                return;
                            });

                    }
                }
            })
            .catch(error => {
                alert("An errored occured whilst opening this batch");
                console.error('Error:', error);
                return;
                //location.reload();
            });


    })

});


function handleUploadFileS(e) {
    var files = e.target.files;
    var filelist = document.getElementById('filelist');
    var total_size = 0;
    filelist.innerHTML = "";
    for (var i = 0; i < files.length; i++) {
        var file = files[i];
        var reader = new FileReader();
        //formData.append('files[]', file);
        total_size = total_size + file.size / 1000;

        reader.onload = (function (file, i) {
            return function () {
                //formData.append('files[]', file);

                var li = document.createElement('li');
                li.setAttribute('data-id', 'file-' + i++);

                var input = document.createElement('input');
                input.setAttribute('name', 'files[]');
                input.setAttribute('id', 'fileArray');
                input.setAttribute('type', 'hidden');

                var div = document.createElement('div');
                div.setAttribute('class', 'file-container');


                var div2 = document.createElement('div');
                div2.setAttribute('class', 'filename');


                div2.appendChild(document.createTextNode(file.name));

                li.appendChild(input);
                li.appendChild(div);
                li.appendChild(div2);


                filelist.prepend(li)
            }
        })(file, i);

        reader.readAsDataURL(file);
    }

    // updated the total file count
    document.querySelector("#files_detected").textContent = `${files.length}`;

    total_size < 1000 ? document.querySelector("#file_size").textContent = `${total_size.toFixed(2)}Kb` : document.querySelector("#file_size").textContent = `${(total_size / 1000).toFixed(2)} Mb`;

    // show files added
    Files = files;
}


function add_data_tag(e) {
    console.log("Added");
    let new_tag = document.createElement("input");
    new_tag.setAttribute("class", "queryvaluetag");
    new_tag.setAttribute("placeholder", "Set data")
    new_tag.setAttribute("type", "text");
    e.target.parentElement.prepend(new_tag);
}

function removeQueryCard(e) {
    e.target.parentElement.remove();
    let query_dash_count = document.querySelector("#queries_detected");

    let query_count = parseInt(query_dash_count.textContent);
    query_dash_count.textContent = `${query_count - 1}`
}

function createQueryCard(e) {
    let card = document.createElement("div");
    card.setAttribute("class", "query_card");

    card.innerHTML = queryType == 0 ? `<button onclick="removeQueryCard(event);" class="removequery">-</button>
    <input type="text" placeholder="Search Title" name="query_title1" id="query_title_1" class="query_title">
    <div class="tags">
      <input type="text" placeholder="Data Tag" class="queryvaluetag">
      <input type="button" value="+" id="csqt" onclick="add_data_tag(event);">
    </div>
  </div>` : `<button onclick="removeQueryCard(event);" class="removequery">-</button>
  <input type="text" placeholder="Search Title" name="query_title1" id="query_title_1" class="query_title">`;
    document.querySelector(".queries").append(card);
    let query_dash_count = document.querySelector("#queries_detected");
    let query_count = parseInt(query_dash_count.textContent);
    query_dash_count.textContent = `${query_count + 1}`
}

function switch_query_type(e) {

    if (e.target.classList.contains("active")) {
        return;
    }
    let queries_cont = document.querySelectorAll(".query_card");
    document.querySelectorAll(".top h4").forEach(f => {
        f.classList.remove("active");
    });
    e.target.classList.add("active");
    queries_cont.forEach(e => {
        e.remove()
    });
    let query_dash_count = document.querySelector("#queries_detected");
    query_dash_count.textContent = `0`;
    if (e.target.textContent == "Title and Data Query") {
        queryType = 0;
    }
    else {
        queryType = 1;
    }
}

function parseQueries() {
    var query = {};
    var total_queries = 0;

    if (queryType == 0) {
        query["TitleData"] = {};
        // T+D query
        document.querySelectorAll(".query_card").forEach(e => {
            var title;
            var data = [];
            e.childNodes.forEach(c => {
                // get the title
                if (c.classList && c.classList.contains("query_title") && c.value.length > 0) {
                    title = c.value.trim();
                    //console.log(title);
                }
                else if (c.classList && c.classList.contains("tags")) {
                    c.childNodes.forEach(d => {
                        if (d.classList && d.classList.contains("queryvaluetag") && d.value.length > 0) {
                            data.push(d.value.trim());
                        }
                    })
                }
            })
            //console.log(title);
            if (title && data.length > 0) {
                total_queries += 1;
                query["TitleData"][title] = data;
                //query["TitleData"].push(datum);
            }
            //console.log(query.TitleData[0]);

        });
    }
    else {
        let keys = [];
        // T+D query
        document.querySelectorAll(".query_card").forEach(e => {
            e.childNodes.forEach(c => {
                // get the title
                if (c.classList && c.classList.contains("query_title") && c.value.length > 0) {
                    keys.push(c.value.trim());
                    //console.log(keys);
                }

            })
        });
        if (keys.length > 0) {
            query["OnlyData"] = keys;
        }

    }
    console.log(query);
    return (query.OnlyData || total_queries > 0) ? query : null;
}

function batchFile() {
    let filelist = Files;
    var container = [];
    var current_list = [];
    var active_list_len = 0;


    for (let index = 0; index < filelist.length; index++) {
        // 1000 = 1kb ; 1000,000= 1mb; 100,000,000 = 100mb,
        if (filelist[index].size > 100000000) {
            return null;
        }

        if (active_list_len < 100000000 /*kilobyte 100mb*/) {
            current_list.push(filelist[index])
            active_list_len = active_list_len + filelist[index].size;

        }
        else {
            container.push(current_list);
            current_list = [];
            current_list.push(filelist[index])
            active_list_len = 0;
        }

    }
    container.push(current_list);

    return container;
}

function test() {

    let file = { "size": 2000000 };
    var filelist = new Array(10000).fill(file);
    console.log(filelist.length);
    let batch = batchFile(filelist);
    console.log(batch);
}

function add_files_to_form(form, files) {
    for (let index = 0; index < files.length; index++) {
        const file = files[index];

        form.append("files[]", file);
    }
    return form;
}