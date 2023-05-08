cp .env.example .env
composer install
touch /var/www/myapp/storage/logs/laravel.log
chmod -R 777 /var/www/myapp/storage
chmod -R 777 /var/www/myapp/bootstrap
php artisan key:generate
php artisan migrate
php artisan db:seed