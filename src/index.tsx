/* @refresh reload */
import { Router, Route, Routes } from "@solidjs/router";
import { render } from "solid-js/web";
import "./App.css";

import Homepage from "./Pages/Homepage";

render(() => 
    <Router>
        <Routes>
            <Route path="/" component={Homepage}/>
        </Routes>
    </Router>, document.getElementById("root") as HTMLElement);
