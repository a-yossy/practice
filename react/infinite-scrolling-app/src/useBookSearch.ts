import { useEffect, useState } from 'react'
import axios, { Canceler } from 'axios'

type Book = {
  title: string
}

type Books = {
  docs: Book[]
}

export default function useBookSearch(query: string, pageNumber: number) {
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<boolean>(false);
  const [books, setBooks] = useState<string[]>([]);
  const [hasMore, setHasMore] = useState<boolean>(false);

  useEffect(() => {
    setBooks([])
  }, [query])

  useEffect(() => {
    setLoading(true);
    setError(false);
    let cancel: Canceler;
    axios.get<Books>('http://openlibrary.org/search.json',{
      params: { q: query, page: pageNumber },
      cancelToken: new axios.CancelToken(c => cancel = c),
    }).then((res) => {
      setBooks(prevBooks => {
        return [...new Set([...prevBooks, ...res.data.docs.map(b => b.title)])]
      })
      setHasMore(res.data.docs.length > 0);
      setLoading(false);
    }).catch(e => {
      if (axios.isCancel(e)) return
      setError(true);
    })
    return () => cancel()
  }, [query, pageNumber])

  return { loading, error, books, hasMore }
}
