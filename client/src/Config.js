const prod = {
  urls: {
    BASE_URL: ''
  }
}

const dev = {
  urls: {
    BASE_URL: 'http://localhost:8000'
  }
}

export const config = process.env.NODE_ENV === 'development' ? dev : prod;
