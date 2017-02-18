/*
 * build and run with:
 * gcc `pkg-config --cflags --libs  gtk+-3.0` -o test src/test.c && ./test
 */

 #include <gtk/gtk.h>

 static GtkTargetEntry my_target_table =
 	{ "text/plain", 0, 0 };

 void get_data(GtkWidget* w, void* ctx,
 		int x, int y, void* d, void* dt, int time) {
 	puts("good");
 }

 int main (int argc, char *argv[])
 {
 	GtkWidget *window;

 	gtk_init (&argc, &argv);

 	window = gtk_window_new (GTK_WINDOW_TOPLEVEL);

 	GtkWidget* event_box = gtk_event_box_new();

 	gtk_container_add(GTK_CONTAINER(window),
 			event_box);

 	gtk_drag_dest_set(
 			event_box,
 			GTK_DEST_DEFAULT_ALL,
 			&my_target_table,
 			1,
 			GDK_ACTION_COPY);

 	g_signal_connect (window, "destroy", G_CALLBACK (gtk_main_quit), NULL);

 	g_signal_connect (event_box, "drag-data-received", G_CALLBACK (get_data), NULL);

 	GtkWidget *label = gtk_label_new(
 			"test");

 	gtk_container_add(GTK_CONTAINER(event_box),
 			label);

 	gtk_widget_show_all (window);

 	gtk_main ();

 	return 0;
 }
