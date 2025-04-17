document.querySelectorAll("nav span").forEach((tab) => {
    tab.addEventListener("click", () => {
        const selected = tab.dataset.tab;

        // troca a aba ativa
        document.querySelectorAll("nav span").forEach((el) => {
            el.classList.remove("active");
        });
        tab.classList.add("active");

        // mostra o conteúdo da aba clicada
        document.querySelectorAll(".tab-content").forEach((el) => {
            el.hidden = true;
        });
        document.getElementById(`${selected}-tab`).hidden = false;
    });
});

const stations = [
    ["Diamond City Radio", "/station/diamondcity"],
    ["Distress Signal", null],
    ["Emergency Frequency RJ1138", null],
    ["Military Frequency AF95", null],
];

const player = document.querySelector("audio");
const list = document.querySelector(".station-list");

for (const [title, address] of stations) {
    const el = document.createElement("li");
    el.innerText = title;
    if (address !== null) {
        el.setAttribute("data-station", address);
    } else {
        el.setAttribute("aria-disabled", true);
    }
    list.appendChild(el);
}

const removeSelection = () =>
    list.querySelector(".active")?.classList.remove("active");

const setSelection = (url) =>
    list.querySelector(`[data-station="${url}"]`)?.classList.add("active");

function playUrl(url) {
    if (player.getAttribute("data-src") === url) {
        removeSelection();
        player.removeAttribute("data-src");
        player.src = "";
    } else {
        removeSelection();
        setSelection(url);
        player.setAttribute("data-src", url);
        player.src = url;
    }
}

list.addEventListener("click", (event) => {
    const target = event.target;
    if (!target.hasAttribute("data-station")) return;
    playUrl(target.getAttribute("data-station"));
});

player.addEventListener("canplay", () => {
    player.play();
});

const canvas = document.getElementById("wave-canvas");
const ctx = canvas.getContext("2d");

const audioContext = new (window.AudioContext || window.webkitAudioContext)();
const analyser = audioContext.createAnalyser();
analyser.fftSize = 1024;

const source = audioContext.createMediaElementSource(player);
source.connect(analyser);
analyser.connect(audioContext.destination);

const bufferLength = analyser.frequencyBinCount;
const dataArray = new Uint8Array(bufferLength);

function drawWaveform() {
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    analyser.getByteTimeDomainData(dataArray);

    ctx.strokeStyle = "#00ff00";
    ctx.lineWidth = 1;
    ctx.beginPath();

    const sliceWidth = canvas.width / bufferLength;
    let x = 0;

    for (let i = 0; i < bufferLength; i++) {
        let y = (dataArray[i] / 255) * canvas.height;
        if (i === 0) {
            ctx.moveTo(x, y);
        } else {
            ctx.lineTo(x, y);
        }
        x += sliceWidth;
    }

    ctx.stroke();
    requestAnimationFrame(drawWaveform);
}

player.addEventListener("play", () => {
    audioContext.resume();
    requestAnimationFrame(drawWaveform);
});