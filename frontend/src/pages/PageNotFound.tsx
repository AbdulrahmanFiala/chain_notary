import { Result } from 'antd'
import { type FC } from 'react'
import { NavLink } from 'react-router'

const PageNotFound: FC = () => {
  return (
    <div className='flex justify-center items-center h-screen'>
      <Result
        status="404"
        title="404"
        subTitle="Sorry, the page you visited does not exist."
        extra={<NavLink to="/">Back Home</NavLink>}
      />
    </div>
  )
}

export default PageNotFound