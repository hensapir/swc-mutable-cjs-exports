export { };
exports.getRedis = getRedis;
import memoize from "p-memoize";
import { getConfig } from "./config";
const getRedis = memoize(async ()=>{
    const config = await getConfig();
    return new Redis(config.redisUrl);
});
