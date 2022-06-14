export default defineEventHandler((event) => {
  deleteCookie(event, 'nmbl_account')
  return null
})
