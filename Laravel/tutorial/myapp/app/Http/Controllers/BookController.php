<?php

namespace App\Http\Controllers;

use Illuminate\Http\Request;
use App\Models\Book;

class BookController extends Controller
{
    public function index()
    {
        $books = Book::all();

        return view('book/index', compact('books'));
    }

    public function edit($id)
    {
        $book = Book::findOrFail($id);

        return view('book/edit', compact('book'));
    }
}
