// const worker = new Worker('./worker.js');
document.getElementById('fileInput').addEventListener('change', handleFileSelect);
document.getElementById('folderInput').addEventListener('change', handleFileSelect);
document.getElementById('zipInput').addEventListener('change', handleZipFile);
document.getElementById('mergeButton').addEventListener('click', mergeAndDownload);


function handleFileSelect(event) {
    const fileList = event.target.files;
    const fileContainer = document.getElementById('fileList');
    const rejectedModal = document.getElementById('rejectedModal');
    const rejectedMessage = document.getElementById('rejectedMessage');

    Array.from(fileList).forEach(file => {
        const reader = new FileReader();
        reader.onload = function (e) {
            const workbook = XLSX.read(e.target.result, { type: 'binary' });
            const sheet = workbook.Sheets[workbook.SheetNames[0]];

            // Check if the first row contains any data
            let isAnyDataInFirstRow = false;
            const range = XLSX.utils.decode_range(sheet['!ref']);
            for (let colIndex = range.s.c; colIndex <= range.e.c; colIndex++) {
                const cellAddress = XLSX.utils.encode_cell({ r: range.s.r, c: colIndex });
                const cellValue = sheet[cellAddress] ? sheet[cellAddress].v : undefined;
                if (cellValue !== undefined && cellValue.toString().trim() !== '') {
                    isAnyDataInFirstRow = true;
                    break;
                }
            }

            // Check if any header cell is empty
            const isAnyHeaderCellEmpty = Array.from({ length: range.e.c + 1 }, (_, colIndex) => {
                const cellAddress = XLSX.utils.encode_cell({ r: range.s.r, c: colIndex });
                const cellValue = sheet[cellAddress] ? sheet[cellAddress].v : undefined;
                return !cellValue || cellValue.toString().trim() === '';
            }).some(Boolean);

            if (!isAnyDataInFirstRow || isAnyHeaderCellEmpty) {
                const message = `${file.name} - Rejected (No data or empty header cell present)`;
                rejectedMessage.textContent = message;
                openModal();
                return; // Skip processing further for this file
            }

            // Continue processing the file if the first row and header cells contain data
            const fileStatus = document.createElement('div');
            fileStatus.classList.add('file-status');

            // Store the file object along with other information
            fileStatus.file = file;

            const fileInfo = document.createElement('span');
            fileInfo.textContent = `${file.name}`;
            fileStatus.appendChild(fileInfo);

            const deleteButton = document.createElement('button');
            deleteButton.textContent = 'Delete';
            deleteButton.className = 'del-btn';
            deleteButton.addEventListener('click', () => {
                // Handle delete functionality here
                fileContainer.removeChild(fileStatus);
            });
            fileStatus.appendChild(deleteButton);

            fileContainer.appendChild(fileStatus);
        };
        reader.readAsBinaryString(file);
        // mergeAndDownload()
    });
}


function handleZipFile(zips) {
    var files = [];

    Array.from(zips).forEach(zipFile => {
        const reader = new FileReader();
        
        reader.onload = function (e) {
            const zipFileContent = e.target.result;

            JSZip.loadAsync(zipFileContent)
                .then(function (zip) {
                    const zipFileList = Object.keys(zip.files);

                    // Display the list of files in the zip file
                    console.log(`Files in ${zipFile.name}:`, zipFileList);

                    // Process each file in the zip
                    zipFileList.forEach(fileName => {
                        const fileData = zip.files[fileName];

                        // Check if the entry is a file (not a directory)
                        if (!fileData.dir) {
                            // Create a Blob from the file data
                            const blob = new Blob([fileData._data], { type: 'application/octet-stream' });

                            // Create a new File object
                            files.push(new File([blob], fileName));

                        }
                    });
                })
                .catch(function (error) {
                    console.error('Error extracting zip file:', error);
                });
        };

        reader.readAsArrayBuffer(zipFile);
    });
    console.log(files);
    return files;
}


function openModal() {
    const rejectedModal = document.getElementById('rejectedModal');
    rejectedModal.style.display = 'block';
}

function closeModal() {
    const rejectedModal = document.getElementById('rejectedModal');
    rejectedModal.style.display = 'none';
}

function mergeAndDownload() {
    // Get all file status elements
    const fileStatusList = document.querySelectorAll('.file-status');

    if (fileStatusList.length === 0) {
        // No files to merge
        alert('No files to merge.');
        return;
    }

    // Create a new workbook to store the merged data
    const mergedWorkbook = XLSX.utils.book_new();

    // Add the header row to the merged workbook
    const headerRow = ['Modified date', 'File Number', 'Serial Number', 'File Number + Serial', 'File Name'];
    XLSX.utils.book_append_sheet(mergedWorkbook, XLSX.utils.aoa_to_sheet([headerRow]), 'MergedSheet');

    // Counter for file number and serial number
    let fileNumber = 1;
    let serialNumber =0;
    // Iterate through each file status element and add its data to the merged workbook
    fileStatusList.forEach(fileStatus => {
        const file = fileStatus.file;
        const reader = new FileReader();

        reader.onload = function (e) {
            const workbook = XLSX.read(e.target.result, { type: 'binary' });

            // Get the modified date
            const modifiedDate = getTodayDate();

            // Check if the workbook has sheets
            if (workbook.SheetNames.length > 0) {
                // Get the first sheet name only
                const firstSheetName = workbook.SheetNames[0];
                const sheet = workbook.Sheets[firstSheetName];

                // Check if the sheet with the same name already exists
                const uniqueSheetName = generateUniqueSheetName(mergedWorkbook, firstSheetName);

                // Append sheet directly to the merged workbook
                XLSX.utils.book_append_sheet(mergedWorkbook, sheet, uniqueSheetName);

                serialNumber++;
                // Add a new row with specific details for each file
                const newRow = [
                    modifiedDate,
                    fileNumber,
                    serialNumber, // Assuming serial number is same as file number for individual sheets
                    `${fileNumber}-${serialNumber}`,
                    '\xa0\xa0\xa0' + firstSheetName // Use the first sheet name as the file name
                ];

                // Add the new row to the merged sheet
                const mergedSheet = mergedWorkbook.Sheets['MergedSheet'];
                XLSX.utils.sheet_add_aoa(mergedSheet, [newRow], { origin: -1 }); // Add the new row at the end

            }

            // Remove the file status element from the UI
            fileStatus.parentNode.removeChild(fileStatus);

            // Check if all files have been processed
            const remainingFiles = document.querySelectorAll('.file-status');
            if (remainingFiles.length === 0) {
                // Trigger the download after all files have been merged
                downloadMergedWorkbook(mergedWorkbook);
            }
        };

        reader.readAsBinaryString(file);
    });

    // Utility function to convert string to ArrayBuffer
    function downloadMergedWorkbook(mergedWorkbook) {
        if (mergedWorkbook.SheetNames.length > 0) {
            // Create a Blob directly from the sheet data
            const sheetData = XLSX.write(mergedWorkbook, { bookType: 'xlsx', type: 'binary' });
            const blob = new Blob([s2ab(sheetData)], { type: 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet' });

            // Create a download link and trigger the download
            const downloadLink = document.createElement('a');
            downloadLink.href = URL.createObjectURL(blob);
            downloadLink.download = 'merged_file.xlsx';
            document.body.appendChild(downloadLink);
            downloadLink.click();
            document.body.removeChild(downloadLink);
        } else {
            alert('Merged workbook is empty.');
        }
    }

    // Utility function to convert string to ArrayBuffer
    function s2ab(s) {
        const buf = new ArrayBuffer(s.length);
        const view = new Uint8Array(buf);
        for (let i = 0; i < s.length; i++) view[i] = s.charCodeAt(i) & 0xFF;
        return buf;
    }
    // Utility function to get today's date in the format YYYY-MM-DD
    function getTodayDate() {
        const today = new Date();
        const year = today.getFullYear();
        const month = String(today.getMonth() + 1).padStart(2, '0');
        const day = String(today.getDate()).padStart(2, '0');
        const hour = String(today.getHours()).padStart(2, '0');
        const minute = String(today.getMinutes()).padStart(2, '0');
        return `${month}${day}${year}${hour}${minute}`;
    }

    // Utility function to generate a unique sheet name in the workbook
    function generateUniqueSheetName(workbook, baseName) {
        let uniqueName = baseName;
        let suffix = 1;

        while (workbook.SheetNames.includes(uniqueName)) {
            uniqueName = `${baseName}_${suffix}`;
            suffix++;
        }

        return uniqueName;
    }
}

