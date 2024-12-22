// ----------------------- Init app functions -----------------------
const rooms = [
    "Round Table",
    "Training Camp",
    "Castle Hall"
];

const selectRoomBlock = document.querySelector(".selectRoomBlock");

let select = document.createElement("select");
rooms.forEach(item => {
    let option = document.createElement("option");
    select.setAttribute("onchange", "getSelectedRoom(event)")
    option.innerHTML = item;
    option.value = item;
    select.append(option);
})

select.addEventListener("change", (event) => {
    select.value = event.target.value;
    console.log(`Selected room: ${event.target.value}`);
    socket.emit("join", getSelectedRoom());

    const chatContainer = document.getElementById("chat-container");
    chatContainer.innerHTML = "";
});

selectRoomBlock.appendChild(select);

// ----------------------- Socket.io functions -----------------------
const socket = io();

socket.connect("http://localhost:3000");

socket.on('connect',function() {
    console.log('Client has connected to the server!');
});

socket.emit("join", getSelectedRoom());

socket.on('message', function(data) {
    console.log('Received a message from the server!', data);

    let messageBlock = document.createElement("div");
    messageBlock.classList.add("message-block");

    let userElement = document.createElement("strong");
    userElement.classList.add("message-user");
    userElement.textContent = `${data.user}: `;

    let messageElement = document.createElement("span");
    messageElement.classList.add("message-text");
    messageElement.textContent = data.text;

    messageBlock.appendChild(userElement);
    messageBlock.appendChild(messageElement);

    const chatContainer = document.getElementById("chat-container");
    chatContainer.appendChild(messageBlock);

    chatContainer.scrollTop = chatContainer.scrollHeight;
});

socket.on('disconnect',function() {
    console.log('The client has disconnected!');
});

// ----------------------- Utils functions -----------------------
function getSelectedRoom() {
    return select.value;
}

function getMessageValue() {
    const messageValue = document.getElementById("message").value;
    if (!messageValue) {
        alert("Message cannot be null");
        return;
    }
    return messageValue;
}

function sendMessage() {
    const roomName = getSelectedRoom();
    const messageValue = getMessageValue();

    let message = {
        room: roomName,
        text: messageValue
    };

    socket.send(message);
    document.getElementById("message").value = "";
    document.getElementById("message").innerHTML = "";
}