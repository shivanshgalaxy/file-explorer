const totalSpace = 16; // Total disk space in GB
const usedSpace = 5.9; // Used disk space in GB

let defaultPath = "/home/ashborn3";

const filePathEl = document.getElementsByClassName("file-path");
const sideBarEl = document.querySelector(".sidebar-file-menu");
const tableBodyEl = document.querySelector(".my-files table tbody");

// access the pre-bundled global API functions
const { invoke } = window.__TAURI__.tauri

// Calculate stroke dash array
const strokeDashArray = `${(usedSpace / totalSpace) * 360} 360`;

// Apply stroke dash array to the meter element
document.querySelector('.meter').style.strokeDasharray = strokeDashArray;

// Update used space text
document.getElementById("usedSpace").textContent = usedSpace + " GB";
document.getElementById("freeSpace").textContent = (totalSpace - usedSpace) + " GB";

// Calculate and display storage percentage in the circle
const storagePercentage = Math.round((usedSpace / totalSpace) * 100);
document.querySelector('text').textContent = storagePercentage + '%';

filePathEl[0].textContent = defaultPath;

function createTableRow(file) {
    const row = document.createElement('tr');

    const nameCell = document.createElement('td');
    nameCell.textContent = file[0];
    row.appendChild(nameCell);

    const sizeCell = document.createElement('td');
    sizeCell.textContent = file[1];
    row.appendChild(sizeCell);

    const dateCell = document.createElement('td');
    dateCell.textContent = file[2];
    row.appendChild(dateCell);

    const typeCell = document.createElement('td');
    typeCell.textContent = file[3];
    row.appendChild(typeCell);

    return row;
}

function createBackRow() {
    const row = document.createElement('tr');
    const cell = document.createElement('td');
    cell.textContent = '.. (Go up one directory)';
    cell.colSpan = 4; // Assuming your table has 4 columns
    row.appendChild(cell);
    return row;
}

invoke('get_files_and_details_as_vector', {path: defaultPath}).then((filedata) => {
    console.log(filedata);
    tableBodyEl.appendChild(createBackRow()); // Add the back row
    filedata.forEach((file) => {
        tableBodyEl.appendChild(createTableRow(file));
    });
});



tableBodyEl.addEventListener('click', (event) => {
    const clickedRow = event.target.closest('tr');
    if (clickedRow) {
        const name = clickedRow.querySelector('td:first-child').textContent;
        if (name === '.. (Go up one directory)') {
            defaultPath = defaultPath.substring(0, defaultPath.lastIndexOf('/'));
        } else {
            defaultPath = `${defaultPath}/${name}`;
        }
        filePathEl[0].textContent = defaultPath;
        invoke('get_files_and_details_as_vector', {path: defaultPath}).then((filedata) => {
            tableBodyEl.innerHTML = '';
            tableBodyEl.appendChild(createBackRow()); // Add the back row
            filedata.forEach((file) => {
                tableBodyEl.appendChild(createTableRow(file));
            });
        });
    }
});

const searchInput = document.querySelector(".search-bar input");

searchInput.addEventListener("keydown", (event) => {
    if (event.key === "Enter") {
        const searchText = searchInput.value;
        invoke("query_hashmap",{name: searchText}).then((filedata) => {
            const sidebarList = document.querySelector('.sidebar-file-menu ul');
            sidebarList.innerHTML = '';
            filedata.forEach((file) => {
                const listItem = document.createElement('li');
                listItem.textContent = file;
                sidebarList.appendChild(listItem);
            });
        });
    }
});