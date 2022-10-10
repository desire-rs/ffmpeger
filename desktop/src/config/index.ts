import { domain } from '../service/hsck'
interface Service {
  name: string;
  source?: string;
  url?: string;
  urls?: Url[];
  default?: boolean;
  m3u8?: boolean;
}
interface Url {
  name: string;
  url: string;
  default?: boolean;
}
export const userAgent = 'Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/96.0.4664.110 Safari/537.36 OPR/82.0.4227.58';

export const TOKEN = 'Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiJ7XCJpZFwiOlwiY29uY2VwdFwiLFwiZGVzY3JpcHRpb25cIjpcImNvbmNlcHQgZGVza3RvcFwifSIsImNvbXBhbnkiOiJIb2xlIiwiZXhwIjoyNTkyMDAwMDAwfQ.9KMyfiF8tzV_eS1t1l5qAHjxlkfOGBUUmiEDxzfnGtA';
export const API = 'http://0.0.0.0:1333/ffmpeg';
export const API_BASE = 'http://0.0.0.0:1333';
