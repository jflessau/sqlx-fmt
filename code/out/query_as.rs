sqlx::query_as!(
    DeviceState,
    r#"
        select
            c.name "care_home!",
           	d.name "device_name!",
            d.id as "device_id!",
           	case when d."deviceStatus" = 'offline' then 'offline' else 'online' end "device_status!",
           	case when d."fallDetectionTargetState" = 'not_running' then 'off' else 'on' end "fall_detection!",
           	case when d."helpCallDetectionTargetState" = 'not_running' then 'off' else 'on' end "help_call_detection!",
           	beds.amount as "active_bed_zones_for_out_of_bed_detection!",
           	doors.amount as "active_door_zones_for_out_of_room_detection!"
                from
           	device d
                inner join
           	care_home c on c.id = d."careHomeID"

            -- beds
            left outer join lateral (
               	select
              		count(*) as amount
                from
                   	jsonb_array_elements("visionAnalyzerConfig"->'zones') AS zone
                where
                    (zone->>'label' = 'bed')
                    and (zone->>'active')::boolean = true
            ) as beds on true

            -- doors
            left outer join lateral   (
           	    select
              		count(*) as amount
                from
               	    jsonb_array_elements("visionAnalyzerConfig"->'zones') AS zone
                where
                    (zone->>'label' = 'door_exit')
                    and (zone->>'active')::boolean = true
            ) as doors on true
            where
       	        c.id = $1
            order by
           	    d.name asc
    "#,
    care_home_id
)
.fetch_all(pool)
.await
