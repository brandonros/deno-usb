const calculateFilename = () => {
  const filenameBase = "deno_usb"
  let filenameSuffix = ".so"
  let filenamePrefix = "lib"
  if (Deno.build.os === "win") {
    filenameSuffix = ".dll"
    filenamePrefix = ""
  } else if (Deno.build.os === "darwin") {
    filenameSuffix = ".dylib"
  }
  return `./target/${Deno.args[0]}/${filenamePrefix}${filenameBase}${filenameSuffix}`
}

// calculate filename
const filename = calculateFilename()
// load plugin
const plugin = Deno.openPlugin(filename)
// get added ops
const { initContext } = Deno.core.ops()
// call added op
const response = Deno.core.dispatch(
  initContext
)
// decode response
const textDecoder = new TextDecoder()
const decodedResponse = textDecoder.decode(response)
// print response
console.log(`initContext response: ${decodedResponse}`)
