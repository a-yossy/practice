require "test_helper"

class ModalControllerTest < ActionDispatch::IntegrationTest
  test "should get show" do
    get modal_show_url
    assert_response :success
  end
end
