export default defineEventHandler((event) => {
  const { account_id } = useQuery(event)

  if (account_id) {
    setCookie(event, 'nmbl_account', account_id.toString())
  }

  return sendRedirect(event, '/')
})
