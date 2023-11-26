import { invoke } from "@tauri-apps/api/tauri"
import { appWindow } from "@tauri-apps/api/window"
import { createSignal } from "solid-js"



function Topbar(props: any)
{
    let search_bar: any

    function get_keywords(keywords: string)
    {
        let keyword = keywords.split(" ")

        return keyword
    }

    return (
        <div data-tauri-drag-region class="relative w-full flex justify-end items-center select-none py-4">
            <div class="flex items-center border rounded-xl absolute left-4 sm:left-1/2 sm:-translate-x-1/2 overflow-hidden">
                <input ref={search_bar} placeholder="Search Manga..." class="outline-none w-full bg-transparent px-2 py-1"></input>
                <svg onClick={() => { props.search_mangas(get_keywords(search_bar.value)) }} class="w-9 h-9 p-1.5 rounded-r-xl border-l hover:bg-[#E1BEE7] transition-all duration-150" xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 32 32"><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 13S3 13 3 16s14 3 14 3v3.453c0 1.74 2.069 2.65 3.351 1.475l7.04-6.454a2 2 0 0 0 0-2.948l-7.04-6.454C19.07 6.896 17 7.806 17 9.546V13Z"/></svg>
            </div>
            <div class="flex items-center gap-1 mr-2">
                <div class="w-8 h-8 hover:bg-[#4A90E2]/50 p-2 transition-all ease-linear rounded-md" onClick={async () => await appWindow.minimize()}>
                    <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 16 16"><path fill="currentColor" d="M14 8v1H3V8h11z"/></svg>
                </div>
                <div class="w-8 h-8 hover:bg-[#4A90E2]/50 p-2 transition-all ease-linear rounded-md" onClick={async () => await appWindow.toggleMaximize()}>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill="currentColor" d="M4.5 3A1.5 1.5 0 0 0 3 4.5v7A1.5 1.5 0 0 0 4.5 13h7a1.5 1.5 0 0 0 1.5-1.5v-7A1.5 1.5 0 0 0 11.5 3h-7Zm0 1h7a.5.5 0 0 1 .5.5v7a.5.5 0 0 1-.5.5h-7a.5.5 0 0 1-.5-.5v-7a.5.5 0 0 1 .5-.5Z"/></svg></div>
                <div class="w-8 h-8 hover:bg-[#E1BEE7]/50 p-2 transition-all ease-linear rounded-md" onClick={async () => appWindow.hide()}>
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24"><path fill="currentColor" d="M18.3 5.71a.996.996 0 0 0-1.41 0L12 10.59L7.11 5.7A.996.996 0 1 0 5.7 7.11L10.59 12L5.7 16.89a.996.996 0 1 0 1.41 1.41L12 13.41l4.89 4.89a.996.996 0 1 0 1.41-1.41L13.41 12l4.89-4.89c.38-.38.38-1.02 0-1.4z"/></svg>
                </div>
            </div>
        </div>
    )
}

export default Topbar