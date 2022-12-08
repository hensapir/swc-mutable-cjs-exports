export { };
Object.defineProperty(exports, "getRedis", {
    writable: true,
    value: getRedis,
    enumerable: true
});
import memoize from "p-memoize";
import { getConfig } from "./config";
const getRedis = memoize(async ()=>{
    const config = await getConfig();
    return new Redis(config.redisUrl);
});
