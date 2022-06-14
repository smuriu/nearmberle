import { parseURL, withProtocol } from "ufo"

export const useHost = () => {
  if (process.server) {
    const { url } = useRequestEvent().req
    const { host, protocol } = parseURL(url)
    return withProtocol(host, protocol)
  }
  else {
    return window.location.origin
  }
}
