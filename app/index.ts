// imports the freshly built wasm-pack dependency package
import { general_request_id} from "../pkg";

const loadWASM = () => {
    // say hello!
    console.log(general_request_id(Date.now()))
}

loadWASM()
