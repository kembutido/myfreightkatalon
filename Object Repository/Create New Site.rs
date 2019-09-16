<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description>create new site for MyFreight</description>
   <name>Create New Site</name>
   <tag></tag>
   <elementGuidId>df6bd442-77cb-412e-a67e-608c6a59097d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;customer_id\&quot;: \&quot;17\&quot;,\n    \&quot;address\&quot;: {\n        \&quot;code\&quot;: \&quot;DVE\&quot;,\n        \&quot;name\&quot;: \&quot;David Test\&quot;,\n        \&quot;address_line_1\&quot;: \&quot;46 Wallaby  way\&quot;,\n        \&quot;sublocality\&quot;: \&quot;\&quot;,\n        \&quot;locality\&quot;: \&quot;Henley Beach\&quot;,\n        \&quot;postcode\&quot;: \&quot;5022\&quot;,\n        \&quot;region\&quot;: \&quot;SA\&quot;,\n        \&quot;country\&quot;: \&quot;Australia\&quot;,\n        \&quot;email\&quot;: \&quot;dave@email.io\&quot;\n    },\n    \&quot;dg_contact_name\&quot;: \&quot;dgcontact name\&quot;,\n    \&quot;dg_contact_phone\&quot;: \&quot;0262410241\&quot;,\n    \&quot;cs_phone_number\&quot;: \&quot;0262410241\&quot;,\n    \&quot;despatch_enabled\&quot;: false\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://qa.teamwilberforce.com/api/customers/17/sites</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>abba7e77-1647-4ce1-b766-c8fd84eda738</id>
      <masked>false</masked>
      <name>Code</name>
   </variables>
   <variables>
      <defaultValue>''</defaultValue>
      <description></description>
      <id>3faa6df6-517f-4f50-bd95-e3d0e88b59de</id>
      <masked>false</masked>
      <name>Name</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
