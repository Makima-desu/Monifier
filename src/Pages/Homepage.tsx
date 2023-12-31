import { Match, Switch, createSignal } from "solid-js"
import { invoke } from "@tauri-apps/api"


import Topbar from "../Components/Topbar"
import { A } from "@solidjs/router"
// import { open } from "@tauri-apps/api/shell"

function Homepage()
{
    const [mangas, setMangas] = createSignal<any>([])
    const [keywords, setKeywords] = createSignal<string[]>([])
    const [search, setSearch] = createSignal<boolean>(false)

    invoke("check_favorite_manga_parallel").then((response: any) =>
    {
        setMangas(response)
    })

    async function search_mangas(keywords: string[])
    {
        await invoke("search_manga", {keywords: keywords}).then((response: any) =>
        {
            setKeywords(keywords)
            setMangas(response)
            setSearch(true)
        })
    }

    return (
        <div class="w-full h-full flex flex-col">
            <Topbar search_mangas={search_mangas}/>
            <div class="flex w-full h-full px-8 pb-4">
                <div class="flex flex-col w-full h-full bg-[#FAFAFA] rounded-lg justify-between p-2">
                    <div class="flex w-full h-full relative overflow-y-auto  scrollbar-hide">
                        <div class="w-full grid grid-cols-1 min-[900px]:grid-cols-2 min-[1300px]:grid-cols-3 absolute gap-4">
                            <Switch fallback={            
                                    <div class="absolute text-center w-full flex justify-center">
                                        <span>No Mangas Found, You can search them with the search bar above</span>
                                    </div>}>
                                <Match when={mangas().length !== 0}>
                                    {mangas().map((manga: any, index: number) =>
                                    {
                                        return (
                                            <div class="flex w-full h-min bg-[#eceae4] rounded transition-all duration-150 max-h-[240px]">
                                                <img class="object-cover min-w-[144px] rounded select-none" src={manga.poster}></img>
                                                <div class="flex flex-col w-full h-full py-2 gap-4 px-4">
                                                    <div class="flex w-full justify-between">
                                                        <span class="text-[#4A90E2]">{manga.manga_type}</span>
                                                        <Switch>
                                                            <Match when={!mangas()[index].favorited && search()}>
                                                                <button onClick={async () => { await invoke("add_manga_to_favorites", {manga: manga}); await invoke("search_manga", {keywords: keywords})  }} class="px-2 py-0.5 rounded-lg bg-[#E1BEE7]/50 hover:bg-[#E1BEE7]/80 transition-all duration-150 text-sm">Add to Favorites</button>
                                                            </Match>
                                                            <Match when={mangas()[index].favorited && search()}>
                                                                <button class="px-2 py-0.5 rounded-lg bg-green-200 transition-all duration-150 cursor-default">Favorited</button>
                                                            </Match>
                                                            <Match when={!search()}>
                                                                <button onClick={ async () => await invoke("remove_from_favorites", {id: manga.id}) } class="px-2 py-0.5 rounded-lg bg-red-200 hover:bg-red-300 transition-all duration-150">Remove from favorites</button>
                                                            </Match>
                                                        </Switch>
                                                    </div>
                                                    <span class="font-bold text-xl">{manga.title}</span>
                                                    <div class="flex flex-col w-full h-full gap-2 select-none transition-all duration-100 place-content-center items-center">
                                                        {manga.chapters.map((chapter: any) =>
                                                        {
                                                            return (
                                                                // onClick={async () => await open(`https://mangafire.to${chapter.chapter_link}`)}
                                                                <A  href={`https://mangafire.to${chapter.chapter_link}`} target="_blank" class="w-full font-bold flex justify-between rounded-xl px-2 py-0.5 bg-[#dad5c8] text-sm transition-all duration-150 hover:bg-[#7f7357]/70 cursor-pointer border-[#7f7357]">
                                                                    <span>Chap {chapter.chapter_number} {chapter.language}</span>
                                                                    <span>{chapter.date}</span>
                                                                </A>
                                                            )

                                                        })}
                                                    </div>
                                                </div>
                                            </div>
                                        )
                                    })}
                                </Match>
                            </Switch>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    )
}

export default Homepage