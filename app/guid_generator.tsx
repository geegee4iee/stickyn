'use client'

import {invoke} from "@tauri-apps/api/tauri";
import {useState} from "react";

export default function GuidGenerator(){
const [guid, setGuid] = useState<string>('')
    const generateGuid = async () => {
        const result = await invoke<string>('generate_guid', { version: 'v4'})
        setGuid(result)
    }

    return (
        <div>
            <h1>Guid Generator</h1>
            <button className='bg-blue-500 px-4 rounded' onClick={generateGuid}>Generate a new Guid</button>
            <p>{guid}</p>
        </div>
    )
}