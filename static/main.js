var queryType = 0;
var Files = [];
var uuid = crypto.randomUUID();
var activeProcess = false;

document.addEventListener('DOMContentLoaded', function () {
    const SearchButton = document.querySelector("#startsearch");
    const downloadButton = document.querySelector("#downloadButton");
    const linkDownloadButton = document.querySelector("#downloadlink");
    const filelist = document.getElementById('filelist');

    downloadButton.setAttribute("disabled", "");

    // Example:
    // var fileArray = document.getElementById('fileArray');
    // fileArray.value = JSON.stringify({});
    document.querySelectorAll('.upload-btn-files').forEach(handler => {
        handler.addEventListener('change', async function (e) {
            handleUploadFileS(e);
        });
    });


    //only when a file has been addded allow the search

    SearchButton.addEventListener("click", async (e) => {
        // if already in progres of another query
        if (activeProcess == true) {
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
        var proc_id = crypto.randomUUID();


        console.log(proc_id);
        //set to searching
        SearchButton.textContent = "Searching";
        SearchButton.setAttribute("disabled", "");
        downloadButton.setAttribute("disabled", "");
        linkDownloadButton.setAttribute("hidden", "");
        activeProcess = true;


        // return;
        let started = new Date()
        console.log(started);
        let listOfForms = comileListOfForms(proc_id, queries);
        await CompileFiles(listOfForms, 10).then(response => {
            console.log(response);

            // handle end of request
            handleEndOfRequest(proc_id)
        }).catch(e => console.log("An error occured while performing operation", e));
        console.log(`Finished in ${new Date() - started} milliseconds`);
    })


    function handleEndOfRequest(proc_id) {
        downloadButton.removeAttribute("disabled");
        linkDownloadButton.href = `/download/${proc_id}`;
        linkDownloadButton.removeAttribute("hidden");
        linkDownloadButton.download = `${getQueryString(0)}-index${getTodayDate()}.xlsx`;
        // change to file save later
        linkDownloadButton.click();

        console.log(activeProcess);
        SearchButton.textContent = "Search";
        SearchButton.removeAttribute("disabled", "");
        activeProcess = false;

        // remove files from memory
        Files = [];
        filelist.innerHTML = "";
        document.querySelector("#file_size").textContent = "0";
        document.getElementById("files_detected").textContent = "0";

        // disable download button after timeout
        setTimeout(() => {
            downloadButton.setAttribute("disabled", "");
            linkDownloadButton.href = ``;
            linkDownloadButton.setAttribute("hidden", "");
        }, 1000 * 60 * 5);
    }

});


async function handleUploadFileS(e) {
    var files = [];
    if (e.target.id == "upload_zip" ) {files = await handleZipFile(e.target.files)} else {files = e.target.files };
    var filelist = document.getElementById('filelist');
    var total_size = 0;
    filelist.innerHTML = "";

    for (var i = 0; i < files.length; i++) {
        var file = files[i];
        var reader = new FileReader();
        //formData.append('files[]', file);
        total_size = total_size + file.size / 1000;
        if (i >= 1000) {
            alert("Your total added files exceed 1000. \n Some of these files may not show in the filebox");
            break;
        }
        if (!(file.name.endsWith('.xls') || file.name.endsWith('.xlsx'))) {
            console.log("Rejected. file", file);
            continue;
        }
        // Use a closure to capture the correct value of i
        (function (index, fileName) {
            reader.onload = function (e) {
                var li = document.createElement('li');
                li.setAttribute('data-id', 'file-' + index);

                var input = document.createElement('input');
                input.setAttribute('name', 'files[]');
                input.setAttribute('id', 'fileArray');
                input.setAttribute('type', 'hidden');

                var div = document.createElement('div');
                div.setAttribute('class', 'file-container');

                var div2 = document.createElement('div');
                div2.setAttribute('class', 'filename');
                div2.appendChild(document.createTextNode(fileName));

                li.appendChild(input);
                li.appendChild(div);
                li.appendChild(div2);

                filelist.prepend(li);
            };
        })(i, file.name);

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
    document.querySelector(".queries").prepend(card);
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
                if (c.classList && c.classList.contains("query_title") && cleanText(c.value).length > 0) {
                    title = cleanText(c.value);
                    //console.log(title);
                }
                else if (c.classList && c.classList.contains("tags")) {
                    c.childNodes.forEach(d => {
                        if (d.classList && d.classList.contains("queryvaluetag") && cleanText(d.value).length > 0) {
                            data.push(cleanText(d.value));
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
                if (c.classList && c.classList.contains("query_title") && cleanText(c.value).length > 0) {
                    keys.push(cleanText(c.value));
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

        if (active_list_len < 100000000 /*kilobyte 100mb*/ && current_list.length < 5000) {
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

// query : Stringified action dictionary
function comileListOfForms(proc_id, query) {
    var listOfForms = [];
    var files_batch = batchFile();

    for (let index = 0; index < files_batch.length; index++) {
        const files = files_batch[index];
        let form = new FormData();
        let id = `${proc_id}@${index}`;
        console.log(id);
        form.append("action", JSON.stringify([id, query]));
        listOfForms.push(add_files_to_form(form, files))
    }

    return listOfForms;
}


async function CompileFiles(listOfForms, MAX_PARALLEL_REQUESTS) {
    let requestQ = [];

    for (let i = 0; i < listOfForms.length; i++) {
        const formData = listOfForms[i];
        console.log(`About to batch request bacth ${i}`);
        var requestPromise;
        if (i == listOfForms.length -1) {
            requestPromise = await fetch('/upload', {
                method: 'POST',
                body: formData
            }).then((response) => {
                // Process request
                console.log(`Success! Response for batch: ${[i]} generated.`);
                return response.json();
            }).catch(e => console.log(`An error occured for this batch ${i}`, e));
        }
        else {
            requestPromise = fetch('/upload', {
                method: 'POST',
                body: formData
            }).then((response) => {
                // Process request
                console.log(`Success! Response for batch: ${[i]} generated.`);
                return response.json();
            }).catch(e => console.log(`An error occured for this batch ${i}`, e));
        }

        requestQ.push(requestPromise);

        if (requestQ.length >= MAX_PARALLEL_REQUESTS) {
            // Wait for all requests in the current batch to complete
            await Promise.all(requestQ);
            requestQ = []; // Clear the batch
        }
    }

    // Wait for any remaining requests to complete
    return Promise.all(requestQ);
};

// utility function to help clean search queries
function cleanText(text) {
    return text.replace(/\s+/g, '');
}

async function handleZipFile(zips) {
    const processedFiles = [];

    for (const zipFile of zips) {
        const fileContent = await new Promise((resolve) => {
            const reader = new FileReader();
            reader.onload = (e) => resolve(e.target.result);
            reader.readAsArrayBuffer(zipFile);
        });

        const zip = await JSZip.loadAsync(fileContent);

        const zipFileList = Object.keys(zip.files);

        // Display the list of files in the zip file
        console.log(`Files in ${zipFile.name}:`, zipFileList);

        zipFileList.forEach((fileName) => {
            const fileData = zip.files[fileName];

            // Check if the entry is a file (not a directory)
            if (!fileData.dir) {
                const blob = new Blob([fileData._data], { type: 'application/octet-stream' });
                const processedFile = new File([blob], fileName);
                processedFiles.push(processedFile);
            }
        });
    }

    alert(`Processed ${processedFiles.length} files from ${zips.length} zip files`);
    return processedFiles;
}


function getTodayDate() {
    const today = new Date();
    const year = today.getFullYear();
    const month = String(today.getMonth() + 1).padStart(2, '0');
    const day = String(today.getDate()).padStart(2, '0');
    const hour = String(today.getHours()).padStart(2, '0');
    const minute = String(today.getMinutes()).padStart(2, '0');
    return `${month}${day}${year}${hour}${minute}`;
}


function getQueryString(query) {
   if (queryType == 0) {
    // handle as query + data search
    
   }

   return "SSDCPU";
}