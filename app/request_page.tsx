'use client'

import {invoke} from "@tauri-apps/api/tauri";
import {useState} from "react";

export default function RequestPage() {
    const [url, setUrl] = useState<string>('')
    const [body, setBody] = useState<string>('')
    const sendRequest = async () => {
        const result = await invoke<string>('send_request', { url, body })
        console.log(result)
    }
    return (
        <div className="flex min-h-screen flex-col items-center justify-between p-24">
            <div>
                <h2>Post a request</h2>
                <input type="text" placeholder="URL" value={url} className="border border-gray-300 p-2 rounded-lg" />
                <textarea placeholder="Body" value={body} className="border border-gray-300 p-2 rounded-lg" />
                <button className="bg-blue-500 px-4 rounded">Send</button>
            </div>
        </div >
    );
}