defmodule RealworldWeb.ArticleLive.Show do
  use RealworldWeb, :live_view

  alias Realworld.Blogs

  @impl true
  def mount(_params, _session, socket) do
    {:ok, socket}
  end

  @impl true
  # def handle_params(%{"id" => id}, _, socket) do
  #   {:noreply,
  #    socket
  #    |> assign(:page_title, page_title(socket.assigns.live_action))
  #    |> assign(:article, Blogs.get_article!(id))}
  # end

  defp page_title(:show), do: "Show Article"
  defp page_title(:edit), do: "Edit Article"

  on_mount {RealworldWeb.CurrentUserAssign, :user}

  def handle_params(%{"id" => id}, _, socket) do
    article = Blogs.get_article!(id)
    %{
      live_action: action,
      current_user: user
    } = socket.assigns
    author_id = article.author_id

    if action == :edit && author_id != user.id do
      redirect_path = Routes.article_show_path(socket, :show, article)
      {:noreply, push_redirect(socket, to: redirect_path)}
    else
      {:noreply, socket |> assign(:article, article)}
    end
  end

  def handle_event("delete", _value, socket) do
    %{
      article: article,
      current_user: user
    } = socket.assigns

    if article.author_id != user.id do
      {:noreply, socket}
    else
      {:ok, _} = Blogs.delete_article(article)
      redirect_path = Routes.article_index_path(socket, :index)
      {:noreply, push_redirect(socket, to: redirect_path)}
    end
  end

  def mount(_params, _session, socket) do
    {:ok, assign(socket, :comment_changeset, change_comment)}
  end

  def handle_event("post_comment", %{"comment" => comment_params}, socket) do
    case create_comment(comment_params, socket) do
      {:ok, _} ->
        article = Blogs.get_article!(socket.assigns.article.id)
        {:noreply, socket |> assign(:article, article) |> assign(:comment_changeset, change_comment)}
        ->
          {:noreply, socket}
    end
  end

  defp change_comment() do
    Blogs.change_comment(%Blogs.Comment{})
  end

  defp create_comment(comment_params, socket) do
    %{
      article: article,
      current_user: user
    } = socket.assigns

    comment_params
    |> Map.put("article_id", article.id)
    |> Map.put("author_id", user.id)
    |> Blogs.create_comment()
  end
end
