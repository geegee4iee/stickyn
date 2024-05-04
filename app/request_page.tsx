'use client'

import {invoke} from "@tauri-apps/api/tauri";
import {useState} from "react";
import {getAll} from "@tauri-apps/api/window";
import {randomUUID} from "node:crypto";
import {NEXT_CACHE_REVALIDATE_TAG_TOKEN_HEADER} from "next/dist/lib/constants";
import Markdown from "react-markdown";

class HttpRequest {
    uri: string;
    body: string;
    http_type: string;
    body_type: string;

    constructor(uri: string, body: string, http_type: string, body_type: string) {
        this.uri = uri;
        this.body = body;
        this.http_type = http_type;
        this.body_type = body_type;
    }
}

export default function RequestPage() {
    const [uri, setUri] = useState<string>('https://google.com')
    const [body, setBody] = useState<string>('{}')
    const [response, setResponse] = useState<string>('')
    const [saveStatus, setSaveStatus] = useState<string>('')
    const [requests, setRequests] = useState<HttpRequest[]>([]);

    const httpTypes = ['GET', 'POST', 'PUT', 'DELETE']
    const [httpType, setHttpType] = useState<string>('GET')
    const bodyType = 'application/json'
    const sendRequest = async () => {
        const result = await invoke<string>('generate_http_request', {uri, body, httpType, bodyType})
        setResponse(result)
    }

    const saveRequest = async () => {
        const request = {
            uri: uri,
            body: body,
            http_type: httpType,
            body_type: bodyType
        }
        const result = await invoke<string>('save_request', {request})
        setSaveStatus(result);
    }

    const getAllRequests = async () => {
        const result = await invoke<HttpRequest[]>('get_all_saved_requests')
        setRequests(result);
    }

    const openWindow = async () => {
        const random_id = Date.now().toString(36) + Math.random().toString().substring(2);
        await invoke<object>('open_docs', {windowId: random_id});
        console.log('opened docs');
    }

    const [markdown, setMarkdown] = useState<string>('# Hello world\n\nThis is a markdown editor\n\n```python\nprint("Hello world")\n```\n\n[Google](https://google.com)')


    return (
        <div className="container">
            <div className="columns-2">
                <div>
                    Tabs
                </div>
                <div>
                    <select onChange={e => setHttpType(httpTypes[e.target.selectedIndex])}>
                        {httpTypes.map((type, index) => <option key={index}>{type}</option>)}
                    </select>

                    <h2>Post a request</h2>
                    <div className="mx-auto">
                        <input type="text" placeholder="URL" onChange={e => setUri(e.target.value)} value={uri}
                               className="border border-gray-300 p-2 rounded-lg"/>
                        <textarea placeholder="Body" value={body} onChange={e => setBody(e.target.value)}
                                  className="border border-gray-300 p-2 rounded-lg"/>
                    </div>
                    <button className="bg-blue-500 px-4 py-1 rounded" onClick={sendRequest}>Send</button>
                    <button className="bg-blue-500 px-4 py-1 rounded" onClick={saveRequest}>Save</button>
                </div>
                <div className="container">
                    <div dangerouslySetInnerHTML={{__html: response}}></div>
                </div>

                <div className="border border-gray-500">
                    {saveStatus}
                </div>

                <button className="rounded bg-green-300 bg-gradient-to-b px-10 py-2 border border-green-800"
                        onClick={getAllRequests}>
                    Get all requests
                </button>
                <div className="overscroll-contain">
                    <ul className="overflow-auto">
                        {requests.map((request, index) => <li key={index}>{request.uri}</li>)}
                    </ul>
                </div>

                <button className="border border-purple-500" onClick={openWindow}>
                    Open a new window
                </button>
            </div>
            <div className="columns-2">
                <div>
                    Edit markdown
                    <textarea rows={50} spellCheck={true} value={markdown}
                              onChange={e => setMarkdown(e.target.value)}></textarea>
                </div>
                <div>
                    <Markdown>{markdown}</Markdown>
                </div>
            </div>
        </div>
    );
}