import { wasm_parse_sdp, wasm_test_message } from "./rsdparsa";
import { booted } from "./rsdparsa_wasm";

function parse_sdp(){ console.log('ok ...'); };

let good_sdp = `v=0
o=mozilla...THIS_IS_SDPARTA-58.0.2 3148249596125698952 0 IN IP4 0.0.0.0
s=-
t=0 0
a=sendrecv
a=group:BUNDLE sdparta_0
a=ice-options:trickle
a=msid-semantic:WMS *
m=audio 49657 UDP/TLS/RTP/SAVPF 109 101
c=IN IP4 54.153.48.119
a=candidate:0 1 UDP 2122252543 10.1.16.3 58774 typ host
a=sendrecv
a=extmap:2 urn:ietf:params:rtp-hdrext:sdes:mid
a=fmtp:109 maxplaybackrate=48000;stereo=1;useinbandfec=1
a=fmtp:101 0-15
a=ice-pwd:a9d36b79776c74ea7a9e50798262f075
a=ice-ufrag:f2f907fe
a=mid:sdparta_0
a=msid:{332de3e1-55ce-4314-91b7-fcb81de2f779} {4dca8a5a-7f27-4cd6-9d25-7c563723d4e1}
a=rtcp-mux
a=rtpmap:109 opus/48000/2
a=rtpmap:101 telephone-event/8000
a=setup:active
a=ssrc:2434787592 cname:{46004a80-6549-438f-9134-c2b088a22ffe}
m=video 0 UDP/TLS/RTP/SAVPF 120
c=IN IP4 0.0.0.0
a=inactive
a=mid:sdparta_1
a=rtpmap:120 VP8/90000`;

let bad_sdp = `v=0
o=mozilla...THIS_IS_SDPARTA-58.0.2 3148249596125698952 0 IN IP4 0.0.0.0
s=-
t=0 0
a=sendrecv
a=group:BUNDLE sdparta_0
a=ice-options:trickle
a=msid-semantic:WMS *
m=audio 49657 UDP/TLS/RTP/SAVPF 109 101
c=IN IP4 54.153.48.119
z=wedontsupportzones
a=candidate:0 1 UDP 2122252543 10.1.16.3 58774 typ host
a=sendrecv
a=extmap:2 urn:ietf:params:rtp-hdrext:sdes:mid
a=fmtp:109 maxplaybackrate=48000;stereo=1;useinbandfec=1
a=fmtp:101 0-15
a=ice-pwd:a9d36b79776c74ea7a9e50798262f075
a=ice-ufrag:f2f907fe
a=mid:sdparta_0
a=msid:{332de3e1-55ce-4314-91b7-fcb81de2f779} {4dca8a5a-7f27-4cd6-9d25-7c563723d4e1}
a=rtcp-mux
a=rtpmap:109 opus/48000/2
a=rtpmap:101 telephone-event/8000
a=setup:active
a=ssrc:2434787592 cname:{46004a80-6549-438f-9134-c2b088a22ffe}
m=video 0 UDP/TLS/RTP/SAVPF 120
c=IN IP4 0.0.0.0
a=inactive
a=mid:sdparta_1
a=rtpmap:120 VP8/90000`;

function outputLines(lines) {
    let outArea = document.getElementById('outArea');
    outArea.innerHTML = '';
    outArea.style = '';
    lines.forEach(line => {
        let li = document.createElement('li');
        li.textContent = line;
        outArea.appendChild(li);
    })
}

booted.then(() => {
  let parse_sdp = (sdp) => {
    let result = wasm_parse_sdp(sdp, true);
    if (result.success()) {
        outputLines([`Version: ${result.ok().version()}`,
                     `Origin: ${result.ok().origin().to_string()}`]);

    } else {
        alert(`Error: ${result.err_message()}`);
        outputLines(bad_sdp.split('\n'));
        let outArea = document.getElementById('outArea');
        let offendingLine = outArea.children[result.err_line_number()];
        offendingLine.style = 'background-color:#F88';
        let message = document.createElement('li');
        message.textContent = result.err_message();
        message.style = 'background-color:#F88; list-style-type: none';
        outArea.insertBefore(message, offendingLine.nextSibling);
    }
  };
  document.getElementById('optA').onclick = () => parse_sdp(good_sdp);
  document.getElementById('optB').onclick = () => parse_sdp(bad_sdp);
  alert(`WASM loaded RSDPARSA: ${wasm_test_message()}`);
});